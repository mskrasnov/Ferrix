# Ferrix Russian translation
# (C) 2025 Michail Krasnov <mskrasnov07@ya.ru>

# SIDEBAR
sidebar-export = Экспорт
sidebar-settings = Настройки
sidebar-about = О программе
sidebar-basic = Основное
sidebar-hardware = Оборудование
sidebar-admin = Администрирование
sidebar-system = Система
sidebar-manage = Обслуживание

# PAGES
page-dashboard = Обзор
page-procs = Процессоры
page-memory = Память
page-storage = Накопители
page-dmi = Таблицы DMI
page-battery = Аккумулятор
page-screen = Экран
page-distro = Дистрибутив
page-users = Пользователи
page-groups = Группы
page-sysmgr = Системный менеджер
page-sysmon = Системный монитор
page-software = Установленное ПО
page-env = Окружение
page-sensors = Сенсоры
page-kernel = Ядро
page-kmods = Модули ядра
page-dev = Разработка
page-sysmisc = Разное
page-settings = Настройки
page-about = О программе
page-export = Мастер экспорта
page-todo = Не реализованный функционал

page-todo-msg = Этот функционал пока не реализован

# ABOUT PAGE
about-hdr = FSM — ещё один системный профайлер для Linux
about-ferrix = Версия Ferrix System Monitor
about-flib = Версия ferrix-lib
about-author-hdr = Автор:
about-feedback-hdr = Фидбек:
about-source-hdr = Исходный код:
about-blog = Блог:
about-author = (C) 2025 Михаил Краснов
about-donate = Вы можете отправить мне донат на карту: 2202 2062 5233 5406 (Сбер; Россия). Спасибо!
about-donate-lbl = Закиньте донат на мой Boosty!
about-support = Поддержать меня!

# BATTERY PAGE
bat-header = Аккумулятор {$name}
bat-unknown-name = <неизвестное имя>
bat-status = Статус
bat-status-ful = Заряжен полностью
bat-status-dis = Разряжается
bat-status-cha = Заряжается
bat-status-noc = Не заряжается
bat-status-non = None
bat-status-unknown = Неизвестно ({$status})
bat-status-isnpresent = Статус не указан!
bat-capacity = Процент заряда
bat-lvl-ful = Заряжен полностью
bat-lvl-nor = Нормальный заряд
bat-lvl-hig = Высокий заряд
bat-lvl-low = Низкий заряд
bat-lvl-cri = Критический заряд!
bat-lvl-non = None
bat-lvl-unk = Неизвестно ({$lbl})
bat-health = Уровень здоровья, %
bat-tech = Технология
bat-cycle-cnt = Количество циклов
bat-volt-min-des = Минимальное проектное напряжение, В
bat-volt-now = Текущее напряжение
bat-power-now = Текущая мощность
bat-energy-full-des = Полная проектная энергия, Вт/ч
bat-energy-full = Полная энергия, Вт/ч
bat-energy-now = Текущая энергия, Вт/ч
bat-model = Модель аккумулятора
bat-manufact = Производитель
bat-serial = Серийный номер

# TABLE HEADERS
hdr-param = Параметр
hdr-value = Значение

# Boolean values
bool-true = ДА
bool-false = НЕТ

# LOADING PAGE
ldr-page-tooltip = Загрузка данных...

# ERROR PAGE
err-page-tooltip = Ошибка загрузки данных!

# CPU PAGE
cpu-vendor = Производитель
cpu-family = Семейство
cpu-model = Модель
cpu-stepping = Stepping
cpu-microcode = Микрокод
cpu-freq = Частота
cpu-cache = Размер кеша L3
cpu-physical-id = Физический ID
cpu-siblings = Siblings
cpu-core-id = ID ядра
cpu-cpu-cores = Число ядер
cpu-apicid = APIC ID
cpu-iapicid = Initial APIC ID
cpu-fpu = FPU
cpu-fpu-e = FPU Exception
cpu-cpuid-lvl = CPUID Level
cpu-wp = WP
cpu-flags = Флаги
cpu-bugs = Баги
cpu-bogomips = BogoMIPS
cpu-clflush = Размер clflush
cpu-cache-align = Выравнивание кеша
cpu-address-size = Размер адресов
cpu-power = Управление питанием
cpu-processor_no = Процессор №{$proc_no}

# DASHBOARD PAGE
dash-proc = Процессор
dash-mem = Память
dash-sys = Система
dash-host = Имя хоста
dash-proc-info = {$name}, {$threads} потоков
dash-mem-used = Использ.: {$used}
dash-mem-total = Всего: {$total}
dash-proc-usage = Нагрузка на ЦП
dash-proc-usg_label = Общая: {$usage}%

# DISTRO PAGE
distro-name = Имя ОС
distro-id = Идентификатор
distro-like = Дериватив от
distro-cpe = Имя CPE
distro-variant = Редакция/вариант
distro-version = Версия
distro-codename = Кодовое имя
distro-build-id = ID сборки
distro-image-id = ID образа
distro-image-ver = Версия образа
distro-homepage = Домашняя страница
distro-docs = Документация
distro-support = Поддержка
distro-bugtracker = Багтрекер
distro-privacy-policy = Политика конфиденциальности
distro-logo = Логотип
distro-def-host = Стандартное имя хоста
distro-sysext-lvl = Уровень поддержки расширений

# DRM PAGE
drm-title = Экран №{$idx}
drm-summary = Общая информация
drm-vparams = Параметры видео
drm-edid-not-found = Данные EDID для экрана №{$idx} не существуют!
drm-not-enabled = Экран №{$idx} выключен!
drm-modes = Поддерживаемые разрешения
drm-mode = Разрешение
drm-manufacturer = Производитель
drm-pcode = Код продукта
drm-snum = Серийный номер
drm-date = Неделя/Год
drm-edid-ver = Версия EDID
drm-edid-rev = Ревизия EDID
drm-size = Размер экрана, см
drm-gamma = Гамма дисплея (стандартная)
drm-signal = Тип сигнала
drm-digital = Цифровой
drm-analog = Аналоговый
drm-bit-depth = Глубина цвета
drm-interface = Видеоинтерфейс

# GROUPS PAGE
groups-group = Группа №{$group_no}
groups-name = Имя группы
groups-id = ID группы
groups-members = Члены группы

# KERNEL PAGE
kmod-name = Имя
kmod-size = Размер
kmod-instances = Экз.
kmod-depends = Зависимости
kmod-state = Состояние
kmod-addrs = Адреса
kernel-summary = Summary
kernel-cmdline = Командная строка
kernel-arch = Архитектура
kernel-version = Версия
kernel-build = Сборка
kernel-pid-max = Макс. число процессов
kernel-threads-max = Макс. число потоков
kernel-user-evs = Макс. число user events
kernel-avail-enthropy = Доступная энтропия
kernel-summary-hdr = Информация о ядре
kernel-mods-hdr = Загруженные модули ядра

# RAM PAGE
ram-total = Всего
ram-free = Свободно
ram-available = Доступно
ram-buffers = Буферы
ram-cached = В кеше
ram-swap-cached = В кеше подкачки
ram-active = Активная
ram-inactive = Неактивная
ram-active-anon = Активная (анонимные)
ram-inactive-anon = Неактивная (анонимные)
ram-active-file = Активная (файл)
ram-inactive-file = Неактивная (файл)
ram-unevictable = Невыгружаемая
ram-locked = Заблокированная
ram-swap-total = Подкачки всего
ram-swap-free = Подкачки свободно
ram-zswap = ZSwap всего
ram-zswapped = ZSwap'пировано
ram-dirty = Грязные страницы
ram-writeback = Активно пишется на диск
ram-anon-pages = Анонимные страницы
ram-mapped = Отображённая память
ram-shmem = Разделяемая память
ram-kreclaimable = Восстанавливаемая ядром
ram-slab = slab
ram-sreclaimable = Востанавливаемый slab
ram-sunreclaim = Невостанавливаемый slab
ram-kernel-stack = Стек ядра
ram-page-tables = Таблицы страниц
ram-sec-page-tables = Доп. таблицы страниц
ram-nfs-unstable = Нестабильный NFS
ram-bounce = Bounce буферы
ram-writeback-tmp = Временные буферы (для FUSE)
ram-commit-limit = Можно выделить (max.)

# SETTINGS PAGE
settings-update-period = Период обновления
settings-uperiod-tip = Укажите период обновления данных (в сек.). Чем выше период обновления, тем ниже нагрузка на ПК.
settings-look = Оформления программы
settings-look-tip = Стиль оформления влияет на цвета интерфейса и шрифта. Выберите то, что нравится вам.
settings-look-select = Выберите нужный стиль оформления:
settings-save = Сохранить

# STYLE LABELS
style-dark = Тёмный
style-light = Светлый

# SYSTEM MISC PAGE
misc-hostname = Имя хоста
misc-loadavg = Средняя нагрузка
misc-uptime = Время работы
misc-uptime-val = работы: {$up}, простоя: {$down}
misc-de = Рабочее окружение
misc-lang = Язык

# SYSTEM MONITOR PAGE
sysmon-x-axis = Число отсчётов по оси X:
sysmon-toggle = Вкл/выкл сложенные графики
sysmon-cpu-hdr = Использование ЦП
sysmon-ram-hdr = Использование ОЗУ
sysmon-cpu-unk = Статистика использования ЦП неизвестна!
sysmon-cpu-brk = Статистика использования ЦП повреждена!

# SYSTEMD PAGE
sysd-hdr-name = Имя
sysd-hdr-descr = Описание
sysd-hdr-load = Загружен
sysd-hdr-actv = Активен
sysd-hdr-work = Работает
sysd-warning = Внимание:
sysd-warn-txt = Увеличьте размер окна для более корректного отображения ряда строк!
sysd-total = Всего сервисов: {$total}

# USERS PAGE
users-name = Имя пользователя
users-id = ID пользователя
users-gid = ID группы
users-gecos = GECOS
users-home = Домашний каталог
users-shell = Оболочка входа
users-hdr = Пользователь №{$id}
