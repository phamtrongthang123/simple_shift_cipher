Project được viết bằng rust. Để compile hãy chạy `cargo build` hoặc `cargo run`. 
# Sample Usage 

## Encryption
Dưới mode encrypt, hãy gõ text ascii cơ bản, không unicode và dấu đặc biệt (trừ space)
```
Encrypt(0) or Decrypt(1): 0
What is your plain text: thang ne
Here is the cipher text '9>-z 3',
                                 1', written in file output.txt
Please remember your key: [0, 17], IV: MwePJZn and block size: 7 
```
## Decryption 
Có file sample, có thể copy thành:
```
Encrypt(0) or Decrypt(1): 1
What is your path to output file (example ./output.txt): output-sample.txt
What is your keys (example 1,2,3): 0,17
What is your IV (example ahihe):MwePJZn
What is your block size (example 10): 7
Answer: thang ne
```