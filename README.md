# Discord RPC Client

## Examples

### Big image, small image, details and one button

![](https://i.imgur.com/6MmBURG.png)

```bash
discordrpc --client-id 942151169185316874 --details 'untypeable nickname' --button-1-text 'Discord RPC written in Rust' --button-1-url 'https://github.com/MedzikUser/discord-rpc' --large-image medzik --large-image-text medzik --small-image archlinux --small-image-text archlinux
```

### Big image, details, state and two buttons

![](https://i.imgur.com/tRbcy40.png)

```bash
discordrpc --client-id 942151169185316874 --details 'untypeable nickname' --state 'MedzikUser' --button-1-text 'Discord RPC written in Rust' --button-1-url 'https://github.com/MedzikUser/discord-rpc' --button-2-text 'GitHub' --button-2-url 'https://github.com/MedzikUser' --large-image medzik --large-image-text medzik
```

## How to install?

### Linux
Download discordrpc-linux from the [releases page](https://github.com/MedzikUser/discordrpc/releases/latest) and run

    chmod +x discordrpc-linux
    ./discordrpc-linux

### Arch Linux
Using yay ([AUR](https://aur.archlinux.org/packages/discordrpc))

    yay -S discordrpc

Alternatively you can add [this repo](https://github.com/archlinux-pkg/packages) and run

    sudo pacman -S discordrpc

### OSX
Download discordrpc-darwin from the [releases page](https://github.com/MedzikUser/discordrpc/releases/latest) and run

    chmod +x discordrpc-darwin
    ./discordrpc-darwin

### Windows
Download discordrpc-windows from the [releases page](https://github.com/MedzikUser/discordrpc/releases/latest) and run

    chmod +x discordrpc-windows
    ./discordrpc-windows

### Compile with Cargo
Make sure you have the latest version of Rust. Then you can run

    cargo install imgurs

## How do I get a client id?
1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Make New Application, name them freely (the name will be displayed in RPC)

![new app](https://i.imgur.com/RMUjPep.png)

3. Go to General Information

![general](https://i.imgur.com/yuQufwT.png)

3. Copy ID

![copy id](https://i.imgur.com/JDHZ6jy.png)

5. Then go to the Rich Presence tab (optional)

![rich presence](https://i.imgur.com/hIB5VEW.png)

6. Add images to be able to display them in RPC (optional)

![add image](https://i.imgur.com/vtEs7v6.png)
