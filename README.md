# Aleo Wallet & Acceleration Service

## How to install

### 1.Install aleo-acceleration-service
- [Download](https://github.com/Aleo123Dev/aleo-acceleration-service/releases) the lastest release of your platform.

#### Mac
- since this is a prerelease, you need to enable application from any source, drag the app to application folder, then run the following command:

```bash
sudo xattr -r -d com.apple.quarantine
sudo xattr -r -d com.apple.quarantine /Applications/aleo-acc-service.app
```

then you can launch the application

#### Window

Just run the Installer `aleo-acc-service_0.0.x_x64_en-US.msi` and launch the application.

## start service

Only thing you need to do is to launch the application,then the service will run on background, you can safely close the window.

## using service

### 2. Install Soter Wallet
- [Download](https://github.com/aleoweb123/soter_aleo_wallet/releases)  *I recommend selecting the latest released version*.

- Extract the v0.1.2.d.zip file, the dist/ directory is generated.
- Open chrome://extensions/
  <img width="1031" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/7b316dcd-3ca3-4a25-b4f3-267daccea056">
- Drag the dist/ folder onto the browser.

### 3. Configure the ACC service
- Default service address is: http://127.0.0.1:18340
- You can customize acc service,like this:
  <img width="609" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/d7309727-200e-464a-a761-f34032a00269">

### 4. Now, when the method is executed, the local service will be called by default.

### 5. Test Result "Send"
  <img width="596" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/c1b255e4-1a65-44b6-bf70-8d680207176b">
