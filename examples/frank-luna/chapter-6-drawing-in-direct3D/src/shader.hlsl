struct ObjectConstants {
	float4x4 gWorldViewProj; 
};

ConstantBuffer<ObjectConstants> gObjConstants : register(b0);
ConstantBuffer<float> gGameTime : register(b1);

struct VertexIn
{
	float3 PosL  : POSITION;
    float4 Color : COLOR;
};

struct VertexOut
{
	float4 PosH  : SV_POSITION;
    float4 Color : COLOR;
};

VertexOut VS(VertexIn vin)
{
	VertexOut vout;
	
	vin.PosL.xy += 0.5f*sin(vinL.Pos.x)*sin(3.0f*gGameTime);
	vin.PosL.z *= 0.6f + 0.4f*sin(2.0f*gGameTime);

	// Transform to homogeneous clip space.
	vout.PosH = mul(float4(vin.PosL, 1.0f), gObjConstants.gWorldViewProj);
	
	// Just pass vertex color into the pixel shader.
    vout.Color = vin.Color;
    
    return vout;
}

float4 PS(VertexOut pin) : SV_Target
{
    return pin.Color;
}
