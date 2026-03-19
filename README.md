# simple tensors rust
Начало учебного мини-проекта по написанию API для работы с многомерными массивами числовых данных, включая:

- Гибкое задание измерений и размеров
- Использование разных числовых типов данных
- Операции суммирования, разницы, произведения и деления
- Последующее повышение эффективности

# Главная цель проекта
Главной целью проекта служит изучение эффективного и быстрого кода на языке программирования Rust, а также общее изучение его основ.

Разрабатывается студентом Рушиком в тесном кабинете своего колледжа для общего саморазвития ツ

# Основная структура проекта:
simple tensors rust/ <br>
├── [Cargo.toml](./Cargo.toml) <br>
├── [Cargo.lock](./Cargo.lock) <br>
├── README.md <br>
├── [src/](./src/) <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;├── [lib.rs](./src/lib.rs)         # Входная точка библиотека <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;├── [main.rs](./src/main.rs)       # Файл для тестирования (временно) <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;├── [tensor.rs](./src/tensor.rs)   # Определение структуры Tensor <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;└── [ops/](./src/ops/)             # Математические операции <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;├── [mod.rs](./src/ops/mod.rs) <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;├── [add.rs](./src/ops/add.rs) <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;├── [sub.rs](./src/ops/sub.rs) <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;├── [mul.rs](./src/ops/mul.rs) <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;└── [README.md](./src/ops/README.md) <br>

# Список задач
- [] [Функционал описанный во вложении ops](./src/ops/)
- [] Транспонирование
    - [X] [ленивое транспонирование (через strides)](./src/tensor.rs#lazy-transpose-algorithm)
    - [] физическое транспонирование
- [] Создание диагональных тензоров
    - [] Диагональная матрица (главная диагональ)
    - [] Диагональная матрица (побочная диагональ)
    - [] Диагональ внутри N-мерных тензоров
- [] Макросы
    - [] Преобразовать поданные элементы в тензор (подобно работе vec!)