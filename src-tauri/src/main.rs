#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    onecut_lib::run()
}



/*

Хочу начать писать файл с логикой расчета цены резки . Как его назватиь ? где его разместить ?

Заработная плата работника: 30 евро в час
Амортизация станка: 6 евро в час
Стоимость электричества: 0.34 евро за кВт/ч
Обслуживание и ремонт станка: 2 евро в час
Расходные материалы: 1 евро в час
Аренда или ипотека: 4 евро в час
Страхование: 1 евро в час
Административные расходы: 8 евро в час
Износ и замена оборудования: 1 евро в час
Обучение персонала: 0.5 евро в час
Непредвиденные расходы: 1 евро в час
Процент прибыли: 30% (0.30)
Налоговая ставка: 30% (0.30)

*/