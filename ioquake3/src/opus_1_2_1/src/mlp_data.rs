use ::libc;

pub use crate::src::opus_1_2_1::src::mlp::MLP;
/* RMS error was 0.280492, seed was 1480478173 */
/* 0.005976 0.031821 (0.280494 0.280492) done */

static mut weights: [libc::c_float; 450] = [
    -0.514624f32,
    0.0234227f32,
    -0.14329f32,
    -0.0878216f32,
    -0.00187827f32,
    -0.0257443f32,
    0.108524f32,
    0.00333881f32,
    0.00585017f32,
    -0.0246132f32,
    0.142723f32,
    -0.00436494f32,
    0.0101354f32,
    -0.11124f32,
    -0.0809367f32,
    -0.0750772f32,
    0.0295524f32,
    0.00823944f32,
    0.150392f32,
    0.0320876f32,
    -0.0710564f32,
    -1.43818f32,
    0.652076f32,
    0.0650744f32,
    -1.54821f32,
    0.168949f32,
    -1.92724f32,
    0.0517976f32,
    -0.0670737f32,
    -0.0690121f32,
    0.00247528f32,
    -0.0522024f32,
    0.0631368f32,
    0.0532776f32,
    0.047751f32,
    -0.011715f32,
    0.142374f32,
    -0.0290885f32,
    -0.279263f32,
    -0.433499f32,
    -0.0795174f32,
    -0.380458f32,
    -0.051263f32,
    0.218537f32,
    -0.322478f32,
    1.06667f32,
    -0.104607f32,
    -4.70108f32,
    0.312037f32,
    0.277397f32,
    -2.71859f32,
    1.70037f32,
    -0.141845f32,
    0.0115618f32,
    0.0629883f32,
    0.0403871f32,
    0.0139428f32,
    -0.00430733f32,
    -0.0429038f32,
    -0.0590318f32,
    -0.0501526f32,
    -0.0284802f32,
    -0.0415686f32,
    -0.0438999f32,
    0.0822666f32,
    0.197194f32,
    0.0363275f32,
    -0.0584307f32,
    0.0752364f32,
    -0.0799796f32,
    -0.146275f32,
    0.161661f32,
    -0.184585f32,
    0.145568f32,
    0.442823f32,
    1.61221f32,
    1.11162f32,
    2.62177f32,
    -2.482f32,
    -0.112599f32,
    -0.110366f32,
    -0.140794f32,
    -0.181694f32,
    0.0648674f32,
    0.0842248f32,
    0.0933993f32,
    0.150122f32,
    0.129171f32,
    0.176848f32,
    0.141758f32,
    -0.271822f32,
    0.235113f32,
    0.0668579f32,
    -0.433957f32,
    0.113633f32,
    -0.169348f32,
    -1.40091f32,
    0.62861f32,
    -0.134236f32,
    0.402173f32,
    1.86373f32,
    1.53998f32,
    -4.32084f32,
    0.735343f32,
    0.800214f32,
    -0.00968415f32,
    0.0425904f32,
    0.0196811f32,
    -0.018426f32,
    -0.000343953f32,
    -0.00416389f32,
    0.00111558f32,
    0.0173069f32,
    -0.00998596f32,
    -0.025898f32,
    0.00123764f32,
    -0.00520373f32,
    -0.0565033f32,
    0.0637394f32,
    0.0051213f32,
    0.0221361f32,
    0.00819962f32,
    -0.0467061f32,
    -0.0548258f32,
    -0.00314063f32,
    -1.18332f32,
    1.88091f32,
    -0.41148f32,
    -2.95727f32,
    -0.521449f32,
    -0.271641f32,
    0.124946f32,
    -0.0532936f32,
    0.101515f32,
    0.000208564f32,
    -0.0488748f32,
    0.0642388f32,
    -0.0383848f32,
    0.0135046f32,
    -0.0413592f32,
    -0.0326402f32,
    -0.0137421f32,
    -0.0225219f32,
    -0.0917294f32,
    -0.277759f32,
    -0.185418f32,
    0.0471128f32,
    -0.125879f32,
    0.262467f32,
    -0.212794f32,
    -0.112931f32,
    -1.99885f32,
    -0.404787f32,
    0.224402f32,
    0.637962f32,
    -0.27808f32,
    -0.0723953f32,
    -0.0537655f32,
    -0.0336359f32,
    -0.0906601f32,
    -0.0641309f32,
    -0.0713542f32,
    0.0524317f32,
    0.00608819f32,
    0.0754101f32,
    -0.0488401f32,
    -0.00671865f32,
    0.0418239f32,
    0.0536284f32,
    -0.132639f32,
    0.0267648f32,
    -0.248432f32,
    -0.0104153f32,
    0.035544f32,
    -0.212753f32,
    -0.302895f32,
    -0.0357854f32,
    0.376838f32,
    0.597025f32,
    -0.664647f32,
    0.268422f32,
    -0.376772f32,
    -1.05472f32,
    0.0144178f32,
    0.179122f32,
    0.0360155f32,
    0.220262f32,
    -0.0056381f32,
    0.0317197f32,
    0.0621066f32,
    -0.00779298f32,
    0.00789378f32,
    0.00350605f32,
    0.0104809f32,
    0.0362871f32,
    -0.157708f32,
    -0.0659779f32,
    -0.0926278f32,
    0.00770791f32,
    0.0631621f32,
    0.0817343f32,
    -0.424295f32,
    -0.0437727f32,
    -0.24251f32,
    0.711217f32,
    -0.736455f32,
    -2.194f32,
    -0.107612f32,
    -0.175156f32,
    -0.0366573f32,
    -0.0123156f32,
    -0.0628516f32,
    -0.0218977f32,
    -0.00693699f32,
    0.00695185f32,
    0.00507362f32,
    0.00359334f32,
    0.0052661f32,
    0.035561f32,
    0.0382701f32,
    0.0342179f32,
    -0.00790271f32,
    -0.0170925f32,
    0.047029f32,
    0.0197362f32,
    -0.0153435f32,
    0.0644152f32,
    -0.36862f32,
    -0.0674876f32,
    -2.82672f32,
    1.34122f32,
    -0.0788029f32,
    -3.47792f32,
    0.507246f32,
    -0.816378f32,
    -0.0142383f32,
    -0.127349f32,
    -0.106926f32,
    -0.0359524f32,
    0.105045f32,
    0.291554f32,
    0.195413f32,
    0.0866214f32,
    -0.066577f32,
    -0.102188f32,
    0.0979466f32,
    -0.12982f32,
    0.400181f32,
    -0.409336f32,
    -0.0593326f32,
    -0.0656203f32,
    -0.204474f32,
    0.179802f32,
    0.000509084f32,
    0.0995954f32,
    -2.377f32,
    -0.686359f32,
    0.934861f32,
    1.10261f32,
    1.3901f32,
    -4.33616f32,
    -0.00264017f32,
    0.00713045f32,
    0.106264f32,
    0.143726f32,
    -0.0685305f32,
    -0.054656f32,
    -0.0176725f32,
    -0.0772669f32,
    -0.0264526f32,
    -0.0103824f32,
    -0.0269872f32,
    -0.00687f32,
    0.225804f32,
    0.407751f32,
    -0.0612611f32,
    -0.0576863f32,
    -0.180131f32,
    -0.222772f32,
    -0.461742f32,
    0.335236f32,
    1.03399f32,
    4.24112f32,
    -0.345796f32,
    -0.594549f32,
    -76.1407f32,
    -0.265276f32,
    0.0507719f32,
    0.0643044f32,
    0.0384832f32,
    0.0424459f32,
    -0.0387817f32,
    -0.0235996f32,
    -0.0740556f32,
    -0.0270029f32,
    0.00882177f32,
    -0.0552371f32,
    -0.00485851f32,
    0.314295f32,
    0.360431f32,
    -0.0787085f32,
    0.110355f32,
    -0.415958f32,
    -0.385088f32,
    -0.272224f32,
    -1.55108f32,
    -0.141848f32,
    0.448877f32,
    -0.563447f32,
    -2.31403f32,
    -0.120077f32,
    -1.49918f32,
    -0.817726f32,
    -0.0495854f32,
    -0.0230782f32,
    -0.0224014f32,
    0.117076f32,
    0.0393216f32,
    0.051997f32,
    0.0330763f32,
    -0.110796f32,
    0.0211117f32,
    -0.0197258f32,
    0.0187461f32,
    0.0125183f32,
    0.14876f32,
    0.0920565f32,
    -0.342475f32,
    0.135272f32,
    -0.168155f32,
    -0.033423f32,
    -0.0604611f32,
    -0.128835f32,
    0.664947f32,
    -0.144997f32,
    2.27649f32,
    1.28663f32,
    0.841217f32,
    -2.42807f32,
    0.0230471f32,
    0.226709f32,
    -0.0374803f32,
    0.155436f32,
    0.0400342f32,
    -0.184686f32,
    0.128488f32,
    -0.0939518f32,
    -0.0578559f32,
    0.0265967f32,
    -0.0999322f32,
    -0.0322768f32,
    -0.322994f32,
    -0.189371f32,
    -0.738069f32,
    -0.0754914f32,
    0.214717f32,
    -0.093728f32,
    -0.695741f32,
    0.0899298f32,
    -2.06188f32,
    -0.273719f32,
    -0.896977f32,
    0.130553f32,
    0.134638f32,
    1.29355f32,
    0.00520749f32,
    -0.0324224f32,
    0.00530451f32,
    0.0192385f32,
    0.00328708f32,
    0.0250838f32,
    0.0053365f32,
    -0.0177321f32,
    0.00618789f32,
    0.00525364f32,
    0.00104596f32,
    -0.0360459f32,
    0.0402403f32,
    -0.0406351f32,
    0.0136883f32,
    0.0880722f32,
    -0.0197449f32,
    0.089938f32,
    0.0100456f32,
    -0.0475638f32,
    -0.73267f32,
    0.037433f32,
    -0.146551f32,
    -0.230221f32,
    -3.06489f32,
    -1.40194f32,
    0.0198483f32,
    0.0397953f32,
    -0.0190239f32,
    0.0470715f32,
    -0.131363f32,
    -0.191721f32,
    -0.0176224f32,
    -0.0480352f32,
    -0.221799f32,
    -0.26794f32,
    -0.0292615f32,
    0.0612127f32,
    -0.129877f32,
    0.00628332f32,
    -0.085918f32,
    0.0175379f32,
    0.0541011f32,
    -0.0810874f32,
    -0.380809f32,
    -0.222056f32,
    -0.508859f32,
    -0.473369f32,
    0.484958f32,
    -2.28411f32,
    0.0139516f32,
    3.90017f32,
    1.71789f32,
    -1.43372f32,
    -2.70839f32,
    1.77107f32,
    5.48006f32,
    1.44661f32,
    2.01134f32,
    -1.88383f32,
    -3.64958f32,
    -1.26351f32,
    0.779421f32,
    2.11357f32,
    3.10409f32,
    1.68846f32,
    -4.46197f32,
    -1.61455f32,
    3.59832f32,
    2.43531f32,
    -1.26458f32,
    0.417941f32,
    1.47437f32,
    2.16635f32,
    -1.909f32,
    -0.828869f32,
    1.38805f32,
    -2.67975f32,
    -0.110044f32,
    1.95596f32,
    0.697931f32,
    -0.313226f32,
    -0.889315f32,
    0.283236f32,
    0.946102f32,
];

static mut topo: [libc::c_int; 3] = [25 as libc::c_int, 16 as libc::c_int, 2 as libc::c_int];
#[no_mangle]

pub static mut net: crate::src::opus_1_2_1::src::mlp::MLP = unsafe {
    {
        let mut init = crate::src::opus_1_2_1::src::mlp::MLP {
            layers: 3 as libc::c_int,
            topo: topo.as_ptr(),
            weights: weights.as_ptr(),
        };
        init
    }
};
