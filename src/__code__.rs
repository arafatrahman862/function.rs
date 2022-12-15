// pub async fn sarve<T>(stream: &mut T) -> Result<()>
// where
//     T: AsyncRead + AsyncWrite + Unpin
// {
//     use tokio::io::AsyncReadExt;
//     loop {
//         let mut buf = [0; 5];
//         stream.read_exact(&mut buf).await?;
//         let [b0, b1, b2, b3, b4] = buf;
//         let id = u16::from_le_bytes([b0, b1]);
//         let data_len: usize = u32::from_le_bytes([b2, b3, b4, 0]).try_into().unwrap();
//         let mut data = vec![0; data_len];
//         stream.read_exact(&mut data).await?;
//         match id {
//             $($id => {
//                 let args = Decoder::decode(&data).unwrap();
//                 std_trait::FnOnce::call_once($func, args).await;
//             }),*
//             _ => return Ok(())
//         }
//     }
// }
