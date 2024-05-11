**TARPAR** - утилита командной строки, предназначенная для анализа **drawio** файла (архитектурной схемы) и генерации текстового списка объектов из схемы. Текстовый список формируется в формате, пригодном для последующей загрузки в табличные редакторы типа MS Excel, LibreOffice Calc и т.п. (тип разделителя полей _точка с запятой_, кодировка _UTF-8_, строковые поля в _двойных кавычках_)

Подробнее про алгоритмы работы утилиты, а также про рекомендуемые приемы создания архитектурных схем редакторе _drawio_ см. в [документации по tarpar](/docs/tarpar-doc.md)

## Использование утилиты

### Linux \ MacOS \ Unix

```bash
tarpar diagram.drawio > diagram.csv
```

### Windows

```cmd
tarpar.exe diagram.drawio > diagram.csv
```

## Сборка утилиты

1. Установить rust (https://www.rust-lang.org/)
2. Клонировать репозиторий и скомпилировать:

```bash
git clone https://github.com/AndrewBiz/tarpar.git
cd tarpar
cargo build --release
```

3. Готовая утилита будет лежать здесь: **./tarpar/target/release/tarpar[.exe]**
