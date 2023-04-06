# busse-845-edi-v2
edi document 845 builder - builds from external csv and utilizes stedi api to generate documents

`cargo build && ./busse-845-edi-v2.exe -c <contract> -b <buyer_file> -s <start YYYY-MM-DD> -e <end YYYY-MM-DD> -o <replacing contract> -p <purpose [new,change,cancel,reissue,renew]>`
