# Discord RPC Client

## Examples

### Big image, small image, details and one button

![discord rpc 1](https://cdn.magicuser.cf/6MmBURG.png)

```bash
discordrpc -c 942151169185316874 -d 'untypeable nickname' --button-1-text 'Discord RPC written in Rust' --button-1-url 'https://github.com/MedzikUser/discord-rpc' -N medzik -I medzik -n archlinux -i archlinux`
```

### Big image, details, state and two buttons

![discord rpc 2](https://cdn.magicuser.cf/tRbcy40.png)

```bash
discordrpc -c 942151169185316874 -d 'untypeable nickname' -s 'MedzikUser' --button-1-text 'Discord RPC written in Rust' --button-1-url 'https://github.com/MedzikUser/discord-rpc' --button-2-text 'GitHub' --button-2-url 'https://github.com/MedzikUser' -N medzik -I medzik
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

![new app](https://cdn.magicuser.cf/RMUjPep.png)

3. Go to General Information

![general](https://cdn.magicuser.cf/yuQufwT.png)

3. Copy ID

![copy id](https://cdn.magicuser.cf/JDHZ6jy.png)

5. Then go to the Rich Presence tab (optional)

![rich presence](https://cdn.magicuser.cf/hIB5VEW.png)

6. Add images to be able to display them in RPC (optional)

![add image](https://cdn.magicuser.cf/vtEs7v6.png)
