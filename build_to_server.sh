server_folder="/root/chat-box";
toml="px.toml";


read -sp "Enter your password: " PASSWORD
echo 
echo "build apps";

temp=$(cat ./Cargo.toml | grep -E "^name\s*=" | awk -F "=" "{print $2}" | tr -d "\"" | tr -d "[:space:]");
package_name=${temp#name=}; 

./build-x86_64-2.sh

echo "send apps to server"
sshpass -p "$PASSWORD" scp -P31111 ./${package_name}-x86_64 root@45.158.126.130:${server_folder}/${package_name}_temp;

echo "prepare"
sshpass -p "$PASSWORD" scp -P31111 ./.config/${toml} root@45.158.126.130:${server_folder}/.config/${toml};

echo "check server"

sshpass -p "$PASSWORD" ssh -p 31111 root@45.158.126.130 "
PID=\$(ps aux | grep '${package_name}' | grep -v 'grep' | awk '{print \$2}');
if [ -n \"\$PID\" ] && ps -p \$PID > /dev/null; then
    kill \$PID
    echo \"Process \$PID killed.\"
else
    echo \"Process not found or already stopped.\"
fi"

echo "run server"
sshpass -p "$PASSWORD" ssh -p 31111 root@45.158.126.130 "mv ${server_folder}/${package_name}_temp ${server_folder}/${package_name}" 
sshpass -p "$PASSWORD" ssh -p 31111 root@45.158.126.130 "chmod 700 ${server_folder}/${package_name} && rm /var/program.log; exit " 
timeout 3s sshpass -p "$PASSWORD" ssh -p 31111 -o ConnectTimeout=3 root@45.158.126.130 "cd ${server_folder} && setsid nohup ./${package_name}> /var/program.log 2>&1 & disown; exit" 
echo "halo"
