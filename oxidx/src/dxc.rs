//! Bindings for the DirectX Shader Compiler (`dxcompiler.dll`), the compiler used for
//! shader model 6.0 and above.
//!
//! This module is gated behind the `dxc` feature. The compiler is loaded at runtime through
//! [`DxcLibrary`], so the binary does not depend on `dxcompiler.dll` being present: a missing
//! library is an ordinary error, and the application can ship the library wherever it likes.
//!
//! ```no_run
//! use oxidx::dx::*;
//!
//! # fn main() -> Result<(), DxError> {
//! // Or `DxcLibrary::open("redist/dxcompiler.dll")` to load a specific library.
//! let library = DxcLibrary::new()?;
//!
//! let compiler = library.create_compiler3()?;
//! let utils = library.create_utils()?;
//! let include_handler = utils.create_default_include_handler()?;
//!
//! let source = "float4 main() : SV_Target { return float4(1.0, 0.0, 0.0, 1.0); }";
//!
//! let result = compiler.compile(
//!     &DxcBuffer::from_utf8(source),
//!     &["-T", "ps_6_0", "-E", "main", DXC_ARG_WARNINGS_ARE_ERRORS],
//!     Some(&include_handler),
//! )?;
//!
//! if let Some(errors) = result.errors()? {
//!     eprintln!("{errors}");
//! }
//!
//! let bytecode: Blob = result.object()?;
//! # Ok(())
//! # }
//! ```

use std::path::Path;

use windows::{
    core::{Interface, GUID, HRESULT, PCSTR, PCWSTR},
    Win32::{
        Foundation::HMODULE,
        Graphics::{
            Direct3D::Dxc::{
                CLSID_DxcCompiler, CLSID_DxcUtils, IDxcBlob, IDxcBlobEncoding, IDxcBlobUtf16,
                IDxcBlobUtf8, IDxcCompiler3, IDxcIncludeHandler, IDxcOperationResult, IDxcResult,
                IDxcUtils,
            },
            Direct3D12::ID3D12ShaderReflection,
        },
        System::LibraryLoader::{GetProcAddress, LoadLibraryW},
    },
};

use crate::{
    blob::Blob, create_type, error::DxError, impl_interface, reflection::ShaderReflection, types::*,
};

type DxcCreateInstanceFn = unsafe extern "system" fn(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut std::ffi::c_void,
) -> HRESULT;

/// A loaded `dxcompiler.dll`, and the entry point into this module.
///
/// The library stays loaded for the rest of the process, so the objects it creates outlive the
/// handle they were created from.
#[derive(Clone, Copy, Debug)]
pub struct DxcLibrary {
    module: HMODULE,
    create_instance_proc: DxcCreateInstanceFn,
}

impl DxcLibrary {
    /// Loads `dxcompiler.dll` from the standard library search path, which begins with the
    /// directory of the executable.
    #[inline]
    pub fn new() -> Result<Self, DxError> {
        Self::open("dxcompiler.dll")
    }

    /// Loads a compiler library from the given path.
    ///
    /// The path is handed to `LoadLibraryW` as-is, so a bare file name is resolved against the
    /// standard library search path, while a relative path is resolved against the current
    /// directory.
    #[inline]
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DxError> {
        use std::os::windows::ffi::OsStrExt;

        let path = path.as_ref();
        let wide = path
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect::<Vec<_>>();

        unsafe {
            let module = LoadLibraryW(PCWSTR::from_raw(wide.as_ptr())).map_err(|err| {
                DxError::Other(format!(
                    "failed to load {}: {}",
                    path.display(),
                    err.message()
                ))
            })?;

            let create_instance = GetProcAddress(
                module,
                PCSTR::from_raw(c"DxcCreateInstance".as_ptr() as *const _),
            )
            .ok_or_else(|| {
                DxError::Other(format!(
                    "{} does not export DxcCreateInstance",
                    path.display()
                ))
            })?;

            Ok(Self {
                module,
                create_instance_proc: std::mem::transmute::<
                    unsafe extern "system" fn() -> isize,
                    DxcCreateInstanceFn,
                >(create_instance),
            })
        }
    }

    /// Returns the module handle of the loaded library.
    #[inline]
    pub fn module(&self) -> HMODULE {
        self.module
    }

    /// Creates a shader compiler.
    ///
    /// For more information: [`DxcCreateInstance function`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-dxccreateinstance)
    #[inline]
    pub fn create_compiler3(&self) -> Result<DxcCompiler3, DxError> {
        self.create_instance(&CLSID_DxcCompiler).map(DxcCompiler3)
    }

    /// Creates the helper object used to make blobs, include handlers and reflection objects.
    ///
    /// For more information: [`DxcCreateInstance function`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-dxccreateinstance)
    #[inline]
    pub fn create_utils(&self) -> Result<DxcUtils, DxError> {
        self.create_instance(&CLSID_DxcUtils).map(DxcUtils)
    }

    /// Creates any other object the library exposes, such as an `IDxcValidator` named by
    /// `CLSID_DxcValidator`.
    ///
    /// For more information: [`DxcCreateInstance function`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-dxccreateinstance)
    #[inline]
    pub fn create_instance<T: Interface>(&self, clsid: &GUID) -> Result<T, DxError> {
        unsafe {
            let mut interface = std::ptr::null_mut();

            (self.create_instance_proc)(clsid, &T::IID, &mut interface)
                .ok()
                .map_err(DxError::from)?;

            Ok(T::from_raw(interface))
        }
    }
}

create_type! {
    /// A compiler that turns HLSL source into a DXIL shader object.
    ///
    /// Create one with [`DxcLibrary::create_compiler3`].
    ///
    /// For more information: [`IDxcCompiler3 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/ns-dxcapi-idxccompiler3)
    DxcCompiler3 wrap IDxcCompiler3
}

create_type! {
    /// The result of a DXC operation: a status code, and the blob it produced.
    ///
    /// For more information: [`IDxcOperationResult interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nn-dxcapi-idxcoperationresult)
    DxcOperationResult wrap IDxcOperationResult
}

create_type! {
    /// The result of a compilation, holding every output the arguments asked for.
    ///
    /// For more information: [`IDxcResult interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nn-dxcapi-idxcresult)
    DxcResult wrap IDxcResult; decorator for DxcOperationResult
}

create_type! {
    /// A buffer of arbitrary length owned by DXC.
    ///
    /// For more information: [`IDxcBlob interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nn-dxcapi-idxcblob)
    DxcBlob wrap IDxcBlob
}

create_type! {
    /// A blob that knows the code page of the text it holds.
    ///
    /// For more information: [`IDxcBlobEncoding interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nn-dxcapi-idxcblobencoding)
    DxcBlobEncoding wrap IDxcBlobEncoding; decorator for DxcBlob
}

create_type! {
    /// A blob holding NUL-terminated UTF-8 text.
    ///
    /// For more information: [`IDxcBlobUtf8 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nn-dxcapi-idxcblobutf8)
    DxcBlobUtf8 wrap IDxcBlobUtf8; decorator for DxcBlobEncoding, DxcBlob
}

create_type! {
    /// A blob holding NUL-terminated UTF-16 text.
    ///
    /// For more information: [`IDxcBlobUtf16 interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nn-dxcapi-idxcblobutf16)
    DxcBlobUtf16 wrap IDxcBlobUtf16; decorator for DxcBlobEncoding, DxcBlob
}

create_type! {
    /// Resolves `#include` directives while a shader is being compiled.
    ///
    /// For more information: [`IDxcIncludeHandler interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nn-dxcapi-idxcincludehandler)
    DxcIncludeHandler wrap IDxcIncludeHandler
}

create_type! {
    /// Helpers for creating the blobs, include handlers and reflection objects that the
    /// compiler works with.
    ///
    /// Create one with [`DxcLibrary::create_utils`].
    ///
    /// For more information: [`IDxcUtils interface`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nn-dxcapi-idxcutils)
    DxcUtils wrap IDxcUtils
}

impl_interface! {
    DxcCompiler3;

    /// Compiles a shader.
    ///
    /// `args` takes the same command line arguments as `dxc.exe` itself, for example
    /// `["-T", "ps_6_0", "-E", "main"]`. The `DXC_ARG_*` constants name the most common ones.
    ///
    /// A shader that fails to compile is reported through [`DxcResult::get_status`] rather than
    /// by this method, which returns [`Ok`] whenever the compiler ran at all, so that
    /// [`DxcResult::errors`] can be read either way. Use [`DxcResult::object`] to get the
    /// bytecode and treat a failed compilation as an error.
    ///
    /// For more information: [`IDxcCompiler3::Compile method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxccompiler3-compile)
    #[inline]
    pub fn compile(
        &self,
        source: &DxcBuffer<'_>,
        args: &[impl AsRef<str>],
        include_handler: Option<&DxcIncludeHandler>,
    ) -> Result<DxcResult, DxError> {
        let mut wide_args = Vec::with_capacity(args.len());
        let mut raw_args = Vec::with_capacity(args.len());

        for arg in args {
            let arg = arg
                .as_ref()
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect::<Vec<_>>();

            // Each argument owns its buffer, so growing `wide_args` never moves the memory the
            // pointer refers to.
            raw_args.push(PCWSTR::from_raw(arg.as_ptr()));
            wide_args.push(arg);
        }

        unsafe {
            let result: IDxcResult = if let Some(include_handler) = include_handler {
                self.0.Compile(&source.0, Some(&raw_args), &include_handler.0)
            } else {
                self.0.Compile(&source.0, Some(&raw_args), None)
            }
            .map_err(DxError::from)?;

            Ok(DxcResult(result))
        }
    }

    /// Disassembles a DXIL shader object into text.
    ///
    /// The text is the [`DxcOutKind::Disassembly`] output of the returned result.
    ///
    /// For more information: [`IDxcCompiler3::Disassemble method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxccompiler3-disassemble)
    #[inline]
    pub fn disassemble(&self, object: &DxcBuffer<'_>) -> Result<DxcResult, DxError> {
        unsafe {
            let result: IDxcResult = self.0.Disassemble(&object.0).map_err(DxError::from)?;

            Ok(DxcResult(result))
        }
    }
}

impl_interface! {
    DxcOperationResult,
    DxcResult;

    /// Returns the status of the operation, turning a failure into an error.
    ///
    /// For more information: [`IDxcOperationResult::GetStatus method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcoperationresult-getstatus)
    #[inline]
    pub fn get_status(&self) -> Result<(), DxError> {
        unsafe {
            let status = self.0.GetStatus().map_err(DxError::from)?;

            status.ok().map_err(DxError::from)
        }
    }

    /// Returns the primary blob the operation produced.
    ///
    /// For more information: [`IDxcOperationResult::GetResult method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcoperationresult-getresult)
    #[inline]
    pub fn get_result(&self) -> Result<DxcBlob, DxError> {
        unsafe {
            self.0.GetResult().map(DxcBlob).map_err(DxError::from)
        }
    }

    /// Returns the warnings and errors the operation emitted.
    ///
    /// For more information: [`IDxcOperationResult::GetErrorBuffer method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcoperationresult-geterrorbuffer)
    #[inline]
    pub fn get_error_buffer(&self) -> Result<DxcBlobEncoding, DxError> {
        unsafe {
            self.0.GetErrorBuffer().map(DxcBlobEncoding).map_err(DxError::from)
        }
    }
}

impl_interface! {
    DxcResult;

    /// Checks whether the compilation produced the given output.
    ///
    /// For more information: [`IDxcResult::HasOutput method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcresult-hasoutput)
    #[inline]
    pub fn has_output(&self, kind: DxcOutKind) -> bool {
        unsafe {
            self.0.HasOutput(kind.as_raw()).as_bool()
        }
    }

    /// Returns one output of the compilation, together with the name DXC gave it.
    ///
    /// The blob can be narrowed to the interface the output actually uses: text outputs such
    /// as [`DxcOutKind::Errors`] hold UTF-8, so `DxcBlobUtf8::try_from(blob)` succeeds for them.
    ///
    /// For more information: [`IDxcResult::GetOutput method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcresult-getoutput)
    #[inline]
    pub fn get_output(&self, kind: DxcOutKind) -> Result<(DxcBlob, Option<DxcBlobUtf16>), DxError> {
        unsafe {
            let mut name = None;
            let mut object = None;

            self.0
                .GetOutput::<IDxcBlob>(kind.as_raw(), &mut name, &mut object)
                .map_err(DxError::from)?;

            let object = object
                .ok_or_else(|| DxError::Fail(format!("dxc returned no blob for {kind:?}")))?;

            Ok((DxcBlob(object), name.map(DxcBlobUtf16)))
        }
    }

    /// Returns the number of outputs the compilation produced.
    ///
    /// For more information: [`IDxcResult::GetNumOutputs method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcresult-getnumoutputs)
    #[inline]
    pub fn get_num_outputs(&self) -> u32 {
        unsafe {
            self.0.GetNumOutputs()
        }
    }

    /// Returns the kind of the output at the given index.
    ///
    /// For more information: [`IDxcResult::GetOutputByIndex method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcresult-getoutputbyindex)
    #[inline]
    pub fn get_output_by_index(&self, index: usize) -> DxcOutKind {
        unsafe {
            self.0.GetOutputByIndex(index as u32).into()
        }
    }

    /// Returns the kind of the output the arguments asked for first.
    ///
    /// For more information: [`IDxcResult::PrimaryOutput method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcresult-primaryoutput)
    #[inline]
    pub fn primary_output(&self) -> DxcOutKind {
        unsafe {
            self.0.PrimaryOutput().into()
        }
    }

    /// Returns the compiler diagnostics, or [`None`] when the compiler had nothing to say.
    ///
    /// This is the [`DxcOutKind::Errors`] output read as text; it holds warnings even when the
    /// compilation succeeded.
    #[inline]
    pub fn errors(&self) -> Result<Option<String>, DxError> {
        if !self.has_output(DxcOutKind::Errors) {
            return Ok(None);
        }

        let (blob, _) = self.get_output(DxcOutKind::Errors)?;
        let blob = DxcBlobUtf8::try_from(blob)?;

        if blob.string_len() == 0 {
            return Ok(None);
        }

        Ok(Some(blob.to_string_lossy().into_owned()))
    }

    /// Returns the compiled shader object, ready to be handed to a pipeline state.
    ///
    /// Fails with [`DxError::ShaderCompilationError`] when the compilation itself failed,
    /// carrying the compiler diagnostics.
    #[inline]
    pub fn object(&self) -> Result<Blob, DxError> {
        if let Err(err) = self.get_status() {
            return Err(match self.errors() {
                Ok(Some(errors)) => DxError::ShaderCompilationError(errors),
                _ => DxError::ShaderCompilationError(err.to_string()),
            });
        }

        let (object, _) = self.get_output(DxcOutKind::Object)?;

        Ok(object.to_blob())
    }
}

impl_interface! {
    DxcBlob,
    DxcBlobEncoding,
    DxcBlobUtf8,
    DxcBlobUtf16;

    /// Returns the bytes the blob holds.
    ///
    /// For more information: [`IDxcBlob::GetBufferPointer method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcblob-getbufferpointer)
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.GetBufferPointer() as *const u8,
                self.0.GetBufferSize(),
            )
        }
    }

    /// Returns the size of the blob in bytes.
    ///
    /// For more information: [`IDxcBlob::GetBufferSize method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcblob-getbuffersize)
    #[inline]
    pub fn len(&self) -> usize {
        unsafe {
            self.0.GetBufferSize()
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Copies the blob into a [`Blob`], detaching it from DXC.
    #[inline]
    pub fn to_blob(&self) -> Blob {
        Blob::copy_from_slice(self.as_bytes())
    }

    /// Borrows the blob as a [`DxcBuffer`], so that it can be fed back into the compiler.
    #[inline]
    pub fn as_buffer(&self, encoding: DxcCodePage) -> DxcBuffer<'_> {
        DxcBuffer::new(self.as_bytes(), encoding)
    }
}

impl_interface! {
    DxcBlobEncoding,
    DxcBlobUtf8,
    DxcBlobUtf16;

    /// Returns the code page of the text in the blob, and whether that code page is known.
    ///
    /// For more information: [`IDxcBlobEncoding::GetEncoding method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcblobencoding-getencoding)
    #[inline]
    pub fn get_encoding(&self) -> Result<(bool, DxcCodePage), DxError> {
        unsafe {
            let mut known = Default::default();
            let mut code_page = Default::default();

            self.0.GetEncoding(&mut known, &mut code_page).map_err(DxError::from)?;

            Ok((known.as_bool(), code_page.into()))
        }
    }
}

impl_interface! {
    DxcBlobUtf8;

    /// Returns the text in the blob, without its NUL terminator.
    ///
    /// For more information: [`IDxcBlobUtf8::GetStringPointer method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcblobutf8-getstringpointer)
    #[inline]
    pub fn as_str(&self) -> Result<&str, DxError> {
        std::str::from_utf8(self.as_text_bytes()).map_err(|err| DxError::Other(err.to_string()))
    }

    /// Returns the text in the blob, replacing anything that is not valid UTF-8.
    #[inline]
    pub fn to_string_lossy(&self) -> std::borrow::Cow<'_, str> {
        String::from_utf8_lossy(self.as_text_bytes())
    }

    /// Returns the length of the text in bytes, not counting the NUL terminator.
    ///
    /// For more information: [`IDxcBlobUtf8::GetStringLength method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcblobutf8-getstringlength)
    #[inline]
    pub fn string_len(&self) -> usize {
        unsafe {
            self.0.GetStringLength()
        }
    }

    #[inline]
    fn as_text_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.GetStringPointer().as_ptr(),
                self.0.GetStringLength(),
            )
        }
    }
}

impl_interface! {
    DxcBlobUtf16;

    /// Returns the UTF-16 code units in the blob, without their NUL terminator.
    ///
    /// For more information: [`IDxcBlobUtf16::GetStringPointer method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcblobutf16-getstringpointer)
    #[inline]
    pub fn as_wide(&self) -> &[u16] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.GetStringPointer().as_ptr(),
                self.0.GetStringLength(),
            )
        }
    }

    /// Returns the text in the blob, replacing any unpaired surrogate.
    #[inline]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_wide())
    }

    /// Returns the length of the text in code units, not counting the NUL terminator.
    ///
    /// For more information: [`IDxcBlobUtf16::GetStringLength method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcblobutf16-getstringlength)
    #[inline]
    pub fn string_len(&self) -> usize {
        unsafe {
            self.0.GetStringLength()
        }
    }
}

impl_interface! {
    DxcIncludeHandler;

    /// Loads the source of an included file.
    ///
    /// For more information: [`IDxcIncludeHandler::LoadSource method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcincludehandler-loadsource)
    #[inline]
    pub fn load_source(&self, filename: impl AsRef<Path>) -> Result<DxcBlob, DxError> {
        let filename: windows::core::HSTRING = filename.as_ref().to_str().unwrap_or("").into();

        unsafe {
            self.0.LoadSource(&filename).map(DxcBlob).map_err(DxError::from)
        }
    }
}

impl_interface! {
    DxcUtils;

    /// Creates a blob holding a copy of `data`.
    ///
    /// For more information: [`IDxcUtils::CreateBlob method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcutils-createblob)
    #[inline]
    pub fn create_blob(
        &self,
        data: &[u8],
        encoding: DxcCodePage,
    ) -> Result<DxcBlobEncoding, DxError> {
        unsafe {
            self.0
                .CreateBlob(data.as_ptr() as *const _, data.len() as u32, encoding.as_raw())
                .map(DxcBlobEncoding)
                .map_err(DxError::from)
        }
    }

    /// Creates a blob holding the contents of a file.
    ///
    /// Pass [`None`] as `encoding` to have the encoding deduced from the byte order mark.
    ///
    /// For more information: [`IDxcUtils::LoadFile method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcutils-loadfile)
    #[inline]
    pub fn load_file(
        &self,
        filename: impl AsRef<Path>,
        encoding: Option<DxcCodePage>,
    ) -> Result<DxcBlobEncoding, DxError> {
        let filename: windows::core::HSTRING = filename.as_ref().to_str().unwrap_or("").into();
        let encoding = encoding.map(|encoding| encoding.as_raw());

        unsafe {
            self.0
                .LoadFile(&filename, encoding.as_ref().map(|encoding| encoding as *const _))
                .map(DxcBlobEncoding)
                .map_err(DxError::from)
        }
    }

    /// Creates an include handler that resolves `#include` directives against the file system.
    ///
    /// For more information: [`IDxcUtils::CreateDefaultIncludeHandler method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcutils-createdefaultincludehandler)
    #[inline]
    pub fn create_default_include_handler(&self) -> Result<DxcIncludeHandler, DxError> {
        unsafe {
            self.0
                .CreateDefaultIncludeHandler()
                .map(DxcIncludeHandler)
                .map_err(DxError::from)
        }
    }

    /// Re-encodes a blob as UTF-8.
    ///
    /// For more information: [`IDxcUtils::GetBlobAsUtf8 method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcutils-getblobasutf8)
    #[inline]
    pub fn get_blob_as_utf8(&self, blob: impl AsRef<DxcBlob>) -> Result<DxcBlobUtf8, DxError> {
        unsafe {
            self.0
                .GetBlobAsUtf8(&blob.as_ref().0)
                .map(DxcBlobUtf8)
                .map_err(DxError::from)
        }
    }

    /// Re-encodes a blob as UTF-16.
    ///
    /// For more information: [`IDxcUtils::GetBlobAsUtf16 method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcutils-getblobasutf16)
    #[inline]
    pub fn get_blob_as_utf16(&self, blob: impl AsRef<DxcBlob>) -> Result<DxcBlobUtf16, DxError> {
        unsafe {
            self.0
                .GetBlobAsUtf16(&blob.as_ref().0)
                .map(DxcBlobUtf16)
                .map_err(DxError::from)
        }
    }

    /// Creates a reflection interface over the [`DxcOutKind::Reflection`] output of a compilation.
    ///
    /// For more information: [`IDxcUtils::CreateReflection method`](https://learn.microsoft.com/en-us/windows/win32/api/dxcapi/nf-dxcapi-idxcutils-createreflection)
    #[inline]
    pub fn create_reflection(&self, data: &DxcBuffer<'_>) -> Result<ShaderReflection, DxError> {
        unsafe {
            let mut interface = std::ptr::null_mut();

            self.0
                .CreateReflection(&data.0, &ID3D12ShaderReflection::IID, &mut interface)
                .map_err(DxError::from)?;

            Ok(ShaderReflection(ID3D12ShaderReflection::from_raw(interface)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SOURCE: &str = "float4 main() : SV_Target { return float4(1.0, 0.0, 0.0, 1.0); }";

    /// Loads the compiler the tests run against, honouring `OXIDX_DXCOMPILER` for a library that
    /// does not sit next to the test executable.
    fn library() -> DxcLibrary {
        match std::env::var("OXIDX_DXCOMPILER") {
            Ok(path) => DxcLibrary::open(path).unwrap(),
            Err(_) => DxcLibrary::new().unwrap(),
        }
    }

    #[test]
    fn compile_test() {
        let library = library();

        let compiler = library.create_compiler3().unwrap();
        let utils = library.create_utils().unwrap();
        let include_handler = utils.create_default_include_handler().unwrap();

        let result = compiler
            .compile(
                &DxcBuffer::from_utf8(SOURCE),
                &["-T", "ps_6_0", "-E", "main"],
                Some(&include_handler),
            )
            .unwrap();

        assert!(result.get_status().is_ok());
        assert!(!result.object().unwrap().is_empty());
    }

    #[test]
    fn compile_error_test() {
        let compiler = library().create_compiler3().unwrap();

        let result = compiler
            .compile(
                &DxcBuffer::from_utf8("not hlsl at all"),
                &["-T", "ps_6_0", "-E", "main"],
                None,
            )
            .unwrap();

        assert!(result.get_status().is_err());
        assert!(result.errors().unwrap().is_some());
        assert!(matches!(
            result.object(),
            Err(DxError::ShaderCompilationError(_))
        ));
    }

    #[test]
    fn disassemble_test() {
        let compiler = library().create_compiler3().unwrap();

        let object = compiler
            .compile(
                &DxcBuffer::from_utf8(SOURCE),
                &["-T", "ps_6_0", "-E", "main"],
                None,
            )
            .unwrap()
            .object()
            .unwrap();

        let result = compiler
            .disassemble(&DxcBuffer::new(&object, DxcCodePage::Acp))
            .unwrap();

        let (text, _) = result.get_output(DxcOutKind::Disassembly).unwrap();
        let text = DxcBlobUtf8::try_from(text).unwrap();

        assert!(text.as_str().unwrap().contains("dx.op"));
    }
}
