# Aleo Wallet & Acceleration Service

## How to install

### 1.Install aleo-acceleration-service
- [download](https://github.com/Aleo123Dev/aleo-acceleration-service/releases) the package from release page.
<img width="1034" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/815b5c5e-98ab-4b0b-bd90-b950c4c6b29a">

- for macos, you need to enable application from any source, drag the app to application folder, then run the following command:

```bash
sudo xattr -r -d com.apple.quarantine
sudo xattr -r -d com.apple.quarantine /Applications/aleo-acc-service.app
```

- Open aleo-acc-service.app <img width="110" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/796dceff-3638-4424-8ff6-0ec43986fc0b"> view:
<img width="800" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/84295aaf-1e8a-4abb-a063-b962bff95042">

### 2. Install Soter Wallet
- [Download](https://github.com/aleoweb123/soter_aleo_wallet/releases) the package from release page.
<img width="1017" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/cf312b20-0868-44d6-b4b9-f10e1aa3cbf9">
- Extract the v0.1.2.d.zip file, the dist/ directory is generated.
- Open chrome://extensions/
  <img width="1031" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/7b316dcd-3ca3-4a25-b4f3-267daccea056">
- Drag the dist/ folder onto the browser.

### 3. Configure the ACC service
- Default service address: http://127.0.0.1:18340
- You can customize acc service,like this:
  <img width="609" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/d7309727-200e-464a-a761-f34032a00269">

### 4. Now, when the method is executed, the local service will be called by default.

### 5. Test "Send" Outcome of Execution
  <img width="596" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/c1b255e4-1a65-44b6-bf70-8d680207176b">

## Todos

- [ ] Windows & Linux adaptation / optimization
- [ ] Cross-Platform compilation
- [ ] Settings (Rocksdb as storage)
- [x] Menu/Tray
- [ ] Encryption
- [ ] Icon
- [x] Auto start
- [ ] Allow lan
- [ ] Proxy

## Ref

- <https://betterprogramming.pub/create-menubar-app-with-tauri-510ab7f7c43d>
