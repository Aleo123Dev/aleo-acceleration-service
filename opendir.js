import { exec } from 'child_process';
import os from 'os'

let platform = os.platform();
let handle_res = (error, stdout, stderr) => {
    if (error) {
        console.error(`error: ${error.message}`);
        return;
    }
    if (stderr) {
        console.log(stdout)
        console.error(stderr);
        return;
    }
}

switch (platform) {
    case 'darwin':
        {
            const folderPath = 'target/universal-apple-darwin/release/bundle/dmg';
            exec(`open "${folderPath}"`, handle_res);
        }
        break;
    case 'win32':
        {
            const folderPath = 'target\\release\\bundle\\msi';
            exec(`explorer ${folderPath}`, handle_res);
        }
        break;
    case 'linux':
        {
            const folderPath = 'target/release/bundle';
            exec(`xdg-open "${folderPath}"`, handle_res);
        }
        break;
}
