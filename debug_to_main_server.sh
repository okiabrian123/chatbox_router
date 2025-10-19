server_folder="/home/macmini/chat-box";
toml="px.toml";
ip="34.128.99.29";

temp=$(cat ./Cargo.toml | grep -E "^name\s*=" | awk -F "=" "{print $2}" | tr -d "\"" | tr -d "[:space:]");
package_name=${temp#name=}; 


eval "$(ssh-agent -s)"
ssh-add ~/.ssh/google-cloud-key

echo "prepare"
ssh -i ~/.ssh/google-cloud-key macmini@${ip} "mkdir -p ${server_folder}/.config/";
scp -i ~/.ssh/google-cloud-key ./.config/${toml} macmini@${ip}:${server_folder}/.config/${toml};

echo "check server"
ssh -i ~/.ssh/google-cloud-key macmini@${ip} "
if sudo pkill -f '${package_name}'; then
    echo "Proses yang cocok dengan '$package_name' telah dihentikan."
else
    echo "Tidak ada proses yang ditemukan atau gagal menghentikan proses."
fi
"

echo "run server"
ssh -i ~/.ssh/google-cloud-key  macmini@${ip} "chmod 700 ${server_folder}/${package_name} && rm ${server_folder}/program.log; exit " 
ssh -i ~/.ssh/google-cloud-key -o ServerAliveInterval=30000 -o ServerAliveCountMax=3 macmini@${ip} "cd ${server_folder} && sudo RUST_LOG=info ./${package_name} "
