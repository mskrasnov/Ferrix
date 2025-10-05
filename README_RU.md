<div align="center">
  <img src="assets/logo.svg" width="200">
  <h1>Ferrix — Швейцарский нож для диагностики оборудования в Linux</h1>
  <p><b>Простая программа для получения информации об аппаратном и программном обеспечении компьютера.</b></p>

  [![Лицензия: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/) [![GitHub Release](https://img.shields.io/github/v/release/mskrasnov/ferrix?logo=github)](https://github.com/mskrasnov/ferrix/releases)
</div>💰

## Что такое Ferrix?

Ferrix — это Rust-крейт и программа для получения информации о программном и аппаратном обеспечении компьютера. Она предназначена для работы на современных дистрибутивах Linux и написана на языке Rust с применением `iced`.

## Функции (`ferrix-lib`)

1. Получение информации о:
    - [X] CPU;
    - [X] RAM;
    - [ ] Накопители;
    - [X] BIOS и материнская плата;
    - [ ] Аккумулятор(ы) ноутбука;
    - [X] Установленный дистрибутив Linux;
    - [ ] Рабочее окружение;
    - [X] Сервисы systemd;
    - [ ] Пакеты flatpak;
2. Конвертация собранных данных в:
    - [X] JSON;
    - [X] XML;
<!-- 3. Reset GNOME Desktop settings; -->

<!--## Installation

### Use Flatpak (recommend)

```bash
flatpak install flathub com.mskrasnov.Ferrix
```

### Use AppImage (for portable builds of Ferrix)

Download `*.AppImage` package (runs anywhere):

1. Grab the latest *stable* `*.AppImage` from [Releases](https://github.com/mskrasnov/Ferrix/releases);
2. Make it executable: `chmod +x Ferrix-*.AppImage`;
3. Run it: `./Ferrix-*.AppImage`-->

<!-- ## Screenshots -->

<!-- <details> -->
  <!-- <summary><b>Show</b></summary> -->

  <!-- <br> -->

<!-- **Dashboard** -->
<!-- ![Dashboard page screenshot](assets/main_page.png) -->

<!-- **OS info** -->
<!-- ![OS page screenshot](assets/os_page.png) -->

<!-- **CPU info** -->
<!-- ![CPU info page](assets/cpu_page.png) -->

<!-- **RAM info** -->
<!-- ![RAM info page](assets/ram_page.png) -->

<!-- **Information about system storage** -->
<!-- ![Storage info page](assets/storage_page.png) -->

<!-- **Motherboard info** -->
<!-- ![Motherboard and BIOS info page](assets/dmi_page.png) -->

<!-- **systemd services info** -->
<!-- ![systemd info](assets/systemd_page.png) -->

<!-- **GNOME settings reset page** -->
<!-- ![Reset settings page](assets/reset_page.png) -->

<!-- **Dark mode** -->
<!-- ![Dark mode pages](assets/dark_mode.png) -->

<!-- </details> -->

## Стек технологий

- **ОС:** Linux с `glibc` и `systemd`;
- **Programming language:** Rust 1.88+ (2024 edition);
- **GUI:** [`iced`](https://iced.rs);
- **Оборудование:** современный ПК или ноутбук;

## ❤️ Поддержать Ferrix

Разработка Ferrix требует времени и сил. Если для вас эта программа оказалась полезной, пожалуйста, поддержите её разработку:

- **Поставьте звезду ⭐ этому репозиторию!** Это позволит другим найти Ferrix на GitHub;
- **Пишите комментарии, вопросы, баг-репорты или предложения** о новом функционале в разделе [issues](https://github.com/mskrasnov/Ferrix/issues/new).
- Если вы из России, **отправьте мне донат 💰** на карту: `2202 2062 5233 5406` (Сбербанк). Это позволит мне сохранить энтузиазм, а также поможет заплатить за интернет, чтобы я смог продолжать работать над Ferrix.
- **Посоветуйте Ferrix** друзьям или на форумах.

## Лицензия

Ferrix — это свободное программное обеспечение, которое распространяется под лицензией **GNU General Public License v3.0**. Смотрите файл [LICENSE](LICENSE) для получения дополнительной информации.
