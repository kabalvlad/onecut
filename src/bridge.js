// src/bridge.js

// Экспортируем объект bridge
window.bridge = {
  // Функция для отправки типа материала в бэкенд
  setTypeMaterial: async function(materialType) {
    console.log('Вызов setTypeMaterial с параметром:', materialType);
    return await window.__TAURI__.core.invoke('set_type_material', { 
      materialType: materialType 
    });
  },

  // Функция для получения типа материала
  getTypeMaterial: async function() {
    console.log('Вызов getTypeMaterial');
    return await window.__TAURI__.core.invoke('get_type_material');
  },

  setTypeCutting: async function(cuttingType) {
    console.log('Вызов setTypeCutting с параметром:', cuttingType);
    return await window.__TAURI__.core.invoke('set_type_cutting', { cuttingType: cuttingType });
  },
  

  // Функция для получения типа резки
  getTypeCutting: async function() {
    console.log('Вызов getTypeCutting');
    return await window.__TAURI__.core.invoke('get_type_cutting');
  },

  // Функция для установки толщины
  setThickness: async function(thickness) {
    console.log('Вызов setThickness с параметром:', thickness);
    return await window.__TAURI__.core.invoke('set_thickness', { thickness });
  },

  // Функция для получения толщины
  getThickness: async function() {
    console.log('Вызов getThickness');
    return await window.__TAURI__.core.invoke('get_thickness');
  },

  // Функция для установки длины реза
  setCutLength: async function(length) {
    console.log('Вызов setCutLength с параметром:', length);
    return await window.__TAURI__.core.invoke('set_cut_length', { length });
  },

  // Функция для получения длины реза
  getCutLength: async function() {
    console.log('Вызов getCutLength');
    return await window.__TAURI__.core.invoke('get_cut_length');
  },

  // Функция для установки точек сгиба
  setBendingPoints: async function(points) {
    console.log('Вызов setBendingPoints с параметром:', points);
    return await window.__TAURI__.core.invoke('set_bending_points', { points: Array.from(points) });
  },

  // Функция для получения точек сгиба
  getBendingPoints: async function() {
    console.log('Вызов getBendingPoints');
    return await window.__TAURI__.core.invoke('get_bending_points');
  },

  // Функция для установки резьб/вставок/цековок
  setThreadsInsertsMats: async function(mats) {
    console.log('Вызов setThreadsInsertsMats с параметром:', mats);
    return await window.__TAURI__.core.invoke('set_threads_inserts_mats', { mats: Array.from(mats) });
  },

  // Функция для получения резьб/вставок/цековок
  getThreadsInsertsMats: async function() {
    console.log('Вызов getThreadsInsertsMats');
    return await window.__TAURI__.core.invoke('get_threads_inserts_mats');
  },

  // Функция для установки количества деталей
  setQuantityParts: async function(quantity) {
    console.log('Вызов setQuantityParts с параметром:', quantity);
    return await window.__TAURI__.core.invoke('set_quantity_parts', { quantity });
  },

  // Функция для получения количества деталей
  getQuantityParts: async function() {
    console.log('Вызов getQuantityParts');
    return await window.__TAURI__.core.invoke('get_quantity_parts');
  },

  // Функция для установки стоимости материала
  setCostMaterial: async function(cost) {
    console.log('Вызов setCostMaterial с параметром:', cost);
    return await window.__TAURI__.core.invoke('set_cost_material', { cost });
  },

  // Функция для получения стоимости материала
  getCostMaterial: async function() {
    console.log('Вызов getCostMaterial');
    return await window.__TAURI__.core.invoke('get_cost_material');
  },

  // Функция для установки маржи
  setMarginDeal: async function(margin) {
    console.log('Вызов setMarginDeal с параметром:', margin);
    return await window.__TAURI__.core.invoke('set_margin_deal', { margin });
  },

  // Функция для получения маржи
  getMarginDeal: async function() {
    console.log('Вызов getMarginDeal');
    return await window.__TAURI__.core.invoke('get_margin_deal');
  },

  // Функция для получения цены одной детали
  getPriceOnePart: async function() {
    console.log('Вызов getPriceOnePart');
    return await window.__TAURI__.core.invoke('get_price_one_part');
  },

  // Функция для получения цены всех деталей
  getPriceAllParts: async function() {
    console.log('Вызов getPriceAllParts');
    return await window.__TAURI__.core.invoke('get_price_all_parts');
  },

  // Функция для расчета цены резки
  calculateCuttingPrice: async function() {
    console.log('Вызов calculateCuttingPrice');
    return await window.__TAURI__.core.invoke('calculate_cutting_price_command');
  }

};

// Для отладки
console.log('Bridge initialized:', window.bridge);
