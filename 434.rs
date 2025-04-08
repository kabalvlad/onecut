/* ===== ОБЩИЕ СТИЛИ И ОСНОВНАЯ СТРУКТУРА ===== */
/* Основной контейнер всего приложения */
.container {
    display: flex;           /* Включаем flexbox для основного контейнера */
    flex-direction: column;  /* Меняем на вертикальное расположение */
    max-width: 1400px;       /* Максимальная ширина контейнера */
    margin: 0 auto;          /* Центрирование контейнера */
    padding: 20px;           /* Внутренние отступы */
    font-family: Arial, sans-serif; /* Основной шрифт */
    position: relative;      /* Для корректного позиционирования дочерних элементов */
    min-height: 100vh;       /* Минимальная высота - полная высота экрана */
    box-sizing: border-box;  /* Учитываем padding в общей ширине */
    gap: 10px;               /* Уменьшенное расстояние между рядами */
}

/* ===== ЗАГОЛОВКИ ===== */
/* Основной заголовок */
h1 {
    text-align: center;      /* Центрирование текста */
    color: #333;             /* Цвет текста */
    margin-bottom: 10px;     /* Уменьшенный отступ снизу */
    font-size: 24px;         /* Размер шрифта */
}

/* ===== РЯДЫ ===== */
/* Первый ряд: два блока рядом */
.row-1 {
    display: flex;           /* Flexbox для горизонтального выравнивания */
    gap: 20px;               /* Расстояние между блоками */
    width: 100%;             /* Полная ширина */
    margin-bottom: 10px;     /* Уменьшенный отступ снизу */
}

/* Второй ряд: четыре секции в одной строке */
.row-2 {
    display: flex;           /* Flexbox для горизонтального выравнивания */
    gap: 20px;               /* Расстояние между элементами (такое же, как в row-1) */
    width: 100%;             /* Полная ширина */
    justify-content: space-between; /* Распределение пространства для выравнивания под блоками первого ряда */
    margin-bottom: 10px;     /* Уменьшенный отступ снизу */
    flex-wrap: nowrap;       /* Запрещаем перенос на новую строку */
}

/* ===== ОБЩИЕ СТИЛИ ДЛЯ ВСЕХ БЛОКОВ ===== */
/* Все блоки в приложении */
.block-left, .block-right, 
.cutting-types, .material-types, .thickness-selector, .cut-length-section,
.bending-section, .threads-section, .other-options, .results-section {
    background: #f8f9fa;     /* Светло-серый фон для всех блоков */
    border-radius: 8px;      /* Скругленные углы */
    box-shadow: 0 2px 4px rgba(0,0,0,0.1); /* Тень для глубины */
    padding: 15px;           /* Внутренние отступы */
    display: flex;           /* Включаем flexbox для центрирования */
    flex-direction: column;  /* Элементы в столбец */
    justify-content: flex-start; /* Выравнивание по верхнему краю */
    align-items: center;     /* Центрирование по горизонтали */
    box-sizing: border-box;  /* Учитываем padding в общей ширине */
    position: relative;      /* Для корректного позиционирования */
}

/* Блоки в первом ряду */
.block-left, .block-right {
    flex: 1;                 /* Занимают равное пространство */
    min-height: 320px;       /* Минимальная высота для блоков первого ряда */
}

/* Блоки во втором ряду */
.bending-section, .threads-section, .other-options, .results-section {
    flex: 1;                 /* Занимают равное пространство */
    min-height: 200px;       /* Минимальная высота для блоков второго ряда */
}

/* Колонки внутри блоков */
.cutting-types, .material-types, .thickness-selector, .cut-length-section {
    flex: 1;                 /* Занимают равное пространство */
    height: 100%;            /* Полная высота */
}

/* Строка внутри блоков для горизонтального расположения элементов */
.block-row {
    display: flex;           /* Flexbox для горизонтального выравнивания */
    gap: 20px;               /* Расстояние между элементами */
    width: 100%;             /* Полная ширина */
    justify-content: center; /* Центрирование по горизонтали */
    align-items: flex-start; /* Выравнивание по верхнему краю */
    height: 100%;            /* Полная высота */
}

/* ===== ЗАГОЛОВКИ БЛОКОВ ===== */
/* Все заголовки в блоках */
.cutting-types h2, .material-types h2, .thickness-selector h2, .cut-length-section h2,
.bending-section h2, .threads-section h2, .other-options h2, .results-section h2 {
    text-align: center;      /* Центрирование текста */
    margin: 0 0 15px 0;      /* Отступы: верх, право, низ, лево */
    height: 30px;            /* Фиксированная высота для всех заголовков */
    display: flex;           /* Flexbox для центрирования */
    align-items: center;     /* Центрирование по вертикали */
    justify-content: center; /* Центрирование по горизонтали */
    width: 100%;             /* Полная ширина */
    font-size: 1.2rem;       /* Размер шрифта */
    color: #333;             /* Цвет текста */
    border-bottom: 1px solid #eee; /* Тонкая линия снизу */
    padding-bottom: 5px;     /* Отступ снизу */
    position: relative;      /* Для корректного позиционирования */
}

/* ===== СТИЛИ ДЛЯ БЛОКОВ ТИПОВ РЕЗКИ И МАТЕРИАЛОВ ===== */
/* Контейнеры для опций типов резки и материалов */
.cutting-options, .material-options {
    display: flex;           /* Flexbox для структуры */
    flex-direction: column;  /* Элементы в столбец */
    gap: 10px;               /* Расстояние между элементами */
    width: 100%;             /* Полная ширина */
    padding: 10px;           /* Внутренние отступы */
    box-sizing: border-box;  /* Учитываем padding в общей ширине */
}

/* Отдельные элементы типов резки и материалов */
.cutting-type, .material-type {
    margin: 5px 0;           /* Вертикальные отступы */
    width: 100%;             /* Полная ширина */
}

/* ===== СТИЛИ ДЛЯ БЛОКОВ ТОЛЩИНЫ И ДЛИНЫ РЕЗА ===== */
/* Контейнеры для элементов ввода толщины и длины */
.thickness-input-container, .file-input-container {
    display: flex;           /* Flexbox для структуры */
    flex-direction: column;  /* Элементы в столбец */
    gap: 10px;               /* Расстояние между элементами */
    width: 100%;             /* Полная ширина */
    align-items: center;     /* Центрирование по горизонтали */
    padding: 10px;           /* Внутренние отступы */
    box-sizing: border-box;  /* Учитываем padding в общей ширине */
}

/* Поля ввода в секциях толщины и длины */
.thickness-input-container input,
.thickness-input-container select,
.manual-input-container input {
    width: 80%;              /* Ширина 80% от контейнера */
    padding: 8px 10px;       /* Внутренние отступы */
    border: 1px solid #ced4da; /* Граница */
    border-radius: 4px;      /* Скругленные углы */
    font-size: 14px;         /* Размер шрифта */
    box-sizing: border-box;  /* Учитываем padding в общей ширине */
}

/* Кнопка очистки */
.clear-button {
    width: 80%;              /* Ширина 80% от контейнера */
    padding: 8px 10px;       /* Внутренние отступы */
    background-color: #ff4444; /* Красный фон */
    color: white;            /* Белый текст */
    border: none;            /* Убираем границу */
    border-radius: 4px;      /* Скругленные углы */
    cursor: pointer;         /* Курсор-указатель */
    font-size: 14px;         /* Размер шрифта */
    text-align: center;      /* Центрирование текста */
    margin-top: 10px;        /* Отступ сверху */
}

/* Эффект при наведении на кнопку очистки */
.clear-button:hover {
    background-color: #ff6666; /* Светлее красный при наведении */
}

/* Селектор режима ввода (файл/ручной) */
.input-mode-selector {
    display: flex;           /* Flexbox для структуры */
    gap: 20px;               /* Расстояние между элементами */
    margin-bottom: 15px;     /* Отступ снизу */
    justify-content: center; /* Центрирование элементов */
    width: 100%;             /* Полная ширина */
}

/* Контейнер для ручного ввода */
.manual-input-container {
    width: 80%;              /* Ширина 80% от контейнера */
}

/* Отображение длины реза */
.cut-length-display {
    margin-top: 15px;        /* Отступ сверху */
    padding: 8px;            /* Внутренние отступы */
    background: #e9ecef;     /* Светло-серый фон */
    border-radius: 4px;      /* Скругленные углы */
    display: flex;           /* Flexbox для структуры */
    justify-content: space-between; /* Распределение по краям */
    align-items: center;     /* Центрирование по вертикали */
    width: 80%;              /* Ширина 80% от контейнера */
}

/* Текст внутри отображения длины реза */
.cut-length-display span {
    font-size: 12px;         /* Размер шрифта */
    color: #333;             /* Цвет текста */
}

/* Значение длины реза (второй span) */
.cut-length-display span:last-child {
    font-weight: bold;       /* Жирный шрифт для значения */
    color: #007bff;          /* Синий цвет для выделения */
}

/* ===== СТИЛИ ДЛЯ БЛОКОВ ГИБКИ И РЕЗЬБЫ ===== */
/* Общие стили для опций в секциях */
.bending-points-options,
.threads-inserts-mats-options {
    display: flex;           /* Flexbox для структуры */
    flex-direction: column;  /* Элементы в столбец */
    gap: 10px;               /* Расстояние между элементами */
    width: 100%;             /* Полная ширина */
    align-items: center;     /* Центрирование содержимого */
}

/* Группа радио-кнопок */
.radio-group {
    display: flex;           /* Flexbox для структуры */
    gap: 15px;               /* Расстояние между элементами */
    justify-content: center; /* Центрирование радио-кнопок */
    width: 100%;             /* Полная ширина */
}

/* Стиль для меток */
.radio-group label {
    display: flex;           /* Flexbox для структуры */
    align-items: center;     /* Центрирование по вертикали */
    cursor: pointer;         /* Курсор-указатель */
}

/* Контейнер для числового ввода */
.number-input-container {
    margin-top: 5px;         /* Отступ сверху */
    display: flex;           /* Flexbox для структуры */
    justify-content: center; /* Центрирование поля ввода */
    width: 100%;             /* Полная ширина */
}

/* Стиль для числового ввода */
.number-input-container input[type="number"] {
    width: 80%;              /* Ширина 80% от контейнера */
    padding: 8px 10px;       /* Внутренние отступы */
    border: 1px solid #ced4da; /* Граница */
    border-radius: 4px;      /* Скругленные углы */
    font-size: 14px;         /* Размер шрифта */
}

/* Стиль для отключенного числового ввода */
.number-input-container input[type="number"]:disabled {
    background-color: #e9ecef; /* Серый фон */
    cursor: not-allowed;     /* Курсор запрета */
}

/* ===== СТИЛИ ДЛЯ БЛОКА ДРУГИХ ОПЦИЙ ===== */
/* Контейнер для содержимого других опций */
.options-content {
    display: flex;           /* Flexbox для структуры */
    flex-direction: column;  /* Элементы в столбец */
    gap: 10px;               /* Расстояние между строками */
    width: 100%;             /* Полная ширина */
}

/* Строка в секции других опций */
.options-row {
    display: flex;           /* Flexbox для структуры */
    gap: 10px;               /* Расстояние между элементами */
    justify-content: space-between; /* Распределение пространства */
    width: 100%;             /* Полная ширина */
}

/* Группа ввода в других опциях */
.options-row .input-group {
    flex: 1;                 /* Растягивается на доступное пространство */
    min-width: 0;            /* Минимальная ширина */
}

/* Поля ввода в других опциях */
.options-row .input-group input {
    width: 100%;             /* Полная ширина */
    padding: 8px 10px;       /* Внутренние отступы */
    border: 1px solid #ced4da; /* Граница */
    border-radius: 4px;      /* Скругленные углы */
    font-size: 14px;         /* Размер шрифта */
    box-sizing: border-box;  /* Учитываем padding в общей ширине */
}

/* Кнопка очистить в других опциях */
.options-row .clear-button {
    height: 32px;            /* Высота как у полей ввода */
    padding: 0 15px;         /* Горизонтальные отступы */
    width: auto;             /* Автоматическая ширина */
    margin-top: 0;           /* Убираем отступ сверху */
    align-self: flex-end;    /* Выравнивание по нижнему краю */
}

/* Метки полей ввода */
.options-row .input-group label {
    font-size: 14px;         /* Размер шрифта */
    margin-bottom: 5px;      /* Отступ снизу */
    display: block;          /* Блочное отображение */
}

/* ===== СТИЛИ ДЛЯ БЛОКА РЕЗУЛЬТАТОВ ===== */
/* Контейнер для результатов */
.results-content {
    display: flex;           /* Flexbox для структуры */
    flex-direction: column;  /* Элементы в столбец */
    gap: 10px;               /* Расстояние между элементами */
    width: 100%;             /* Полная ширина */
    align-items: center;     /* Центрирование содержимого */
}

/* Группа результата */
.result-group {
    width: 100%;             /* Полная ширина */
    display: flex;           /* Flexbox для структуры */
    flex-direction: column;  /* Элементы в столбец */
    gap: 5px;                /* Расстояние между элементами */
}

/* Метка результата */
.result-group label {
    font-size: 14px;         /* Размер шрифта */
    color: #555;             /* Цвет текста */
    text-align: center;      /* Центрирование текста */
}

/* Значение результата */
.result-value {
    padding: 10px;           /* Внутренние отступы */
    background: white;       /* Белый фон */
    border-radius: 4px;      /* Скругленные углы */
    font-size: 16px;         /* Размер шрифта */
    font-weight: bold;       /* Жирный шрифт */
    color: #007bff;          /* Синий цвет текста */
    text-align: center;      /* Центрирование текста */
    width: 100%;             /* Полная ширина */
    box-sizing: border-box;  /* Учитываем padding в общей ширине */
    box-shadow: 0 1px 3px rgba(0,0,0,0.1); /* Легкая тень */
}

/* ===== ИНФОРМАЦИОННОЕ ОКНО И ИСТОРИЯ ===== */
/* Информационный блок */
.info-box {
    width: 100%;             /* Полная ширина */
    background: #f5f5f5;     /* Светло-серый фон */
    padding: 20px;           /* Внутренние отступы */
    border-radius: 8px;      /* Скругленные углы */
    box-shadow: 0 2px 8px rgba(0,0,0,0.1); /* Тень для глубины */
    margin-top: 10px;        /* Уменьшенный отступ сверху */
}

/* Список сообщений истории */
.history-list {
    display: flex;           /* Flexbox для структуры */
    flex-direction: column;  /* Расположение элементов по вертикали */
    gap: 10px;               /* Расстояние между сообщениями */
    max-height: 200px;       /* Максимальная высота для 5 сообщений */
    overflow-y: auto;        /* Вертикальная прокрутка при переполнении */
}

/* Отдельные сообщения в истории */
.history-list p {
    margin: 0;               /* Убираем внешние отступы */
    padding: 12px;           /* Внутренние отступы */
    background: white;       /* Белый фон */
    border-radius: 4px;      /* Скругленные углы */
    border-left: 3px solid #007bff; /* Синяя полоса слева для акцента */
    font-size: 14px;         /* Размер шрифта */
    line-height: 1.4;        /* Межстрочный интервал */
    /* Улучшения для отображения длинного текста */
    white-space: pre-wrap;   /* Сохраняет переносы строк и обрабатывает пробелы */
    word-wrap: break-word;   /* Разрыв длинных слов */
    overflow-wrap: break-word; /* Современная версия word-wrap */
}

/* Показываем только последние 5 сообщений по умолчанию */
.history-list p:nth-child(n+6) {
    display: none;
}

/* Когда список активен (при скролле), показываем все сообщения */
.history-list:hover p:nth-child(n+6),
.history-list:focus p:nth-child(n+6),
.history-list:active p:nth-child(n+6) {
    display: block;
}

/* ===== АДАПТИВНОСТЬ ===== */
/* Экраны шириной до 1200px (для планшетов) */
@media (max-width: 1200px) {
    .row-1, .row-2 {
        flex-direction: column; /* Меняем направление на вертикальное */
        gap: 20px;           /* Расстояние между элементами */
    }
    
    .block-row {
        flex-direction: column; /* Меняем направление на вертикальное */
        gap: 20px;           /* Расстояние между элементами */
    }
    
    .cutting-types, .material-types, .thickness-selector, .cut-length-section {
        width: 100%;         /* Полная ширина */
    }
    
    .bending-section, .threads-section, .other-options {
        width: 100%;         /* Полная ширина */
    }
}

/* Экраны шириной до 768px (для мобильных) */
@media (max-width: 768px) {
    .results-row {
        flex-direction: column; /* Меняем направление на вертикальное */
        gap: 15px;           /* Расстояние между элементами */
    }
    
    .result-group {
        width: 100%;         /* Полная ширина */
    }
    
    .input-group {
        width: 100%;         /* Полная ширина */
    }
    
    .radio-group {
        flex-wrap: wrap;     /* Перенос на новую строку при необходимости */
    }
}
