server_folder="/home/macmini/chat-box";
toml="px.toml";
ip="34.128.99.29";

eval "$(ssh-agent -s)"
ssh-add ~/.ssh/google-cloud-key

temp=$(cat ./Cargo.toml | grep -E "^name\s*=" | awk -F "=" "{print $2}" | tr -d "\"" | tr -d "[:space:]");
package_name=${temp#name=}; 

./build-x86_64-2.sh

echo "send apps to server"
scp -i ~/.ssh/google-cloud-key ./${package_name}-x86_64 macmini@${ip}:${server_folder}/${package_name}_temp;



echo "prepare"
ssh -i ~/.ssh/google-cloud-key macmini@${ip} "mkdir -p ${server_folder}/.config/";
scp -i ~/.ssh/google-cloud-key ./.config/${toml} macmini@${ip}:${server_folder}/.config/${toml};

ssh -i ~/.ssh/google-cloud-key macmini@${ip} "
PID=\$(ps aux | grep '${package_name}' | grep -v 'grep' | awk '{print \$2}');
if [ -n \"\$PID\" ] && ps -p \$PID > /dev/null; then
    sudo kill \$PID
    echo \"Process \$PID killed.\"
else
    echo \"Process not found or already stopped.\"
fi"

echo "run server"
ssh -i ~/.ssh/google-cloud-key macmini@${ip} "mv ${server_folder}/${package_name}_temp ${server_folder}/${package_name}" 
ssh -i ~/.ssh/google-cloud-key macmini@${ip} "chmod 700 ${server_folder}/${package_name} && rm /${server_folder}/program.log; exit " 
timeout 3s ssh -i ~/.ssh/google-cloud-key -o ConnectTimeout=3 macmini@${ip} "cd ${server_folder} && sudo setsid nohup ./${package_name}> /${server_folder}/program.log 2>&1 & disown; exit" 
echo "halo"
