import { ref, provide, reactive, inject } from 'vue';

// 创建全局卡片状态管理器
const globalCardStateSymbol = Symbol('globalCardState');

export function createGlobalCardState() {
  const isAllCardsExpanded = ref(false);
  const cardStates = reactive(new Map()); // 存储每个卡片的独立状态

  // 切换所有卡片的展开状态
  const toggleAllCards = () => {
    isAllCardsExpanded.value = !isAllCardsExpanded.value;

    // 更新所有卡片状态
    for (let [key, value] of cardStates) {
      cardStates.set(key, !isAllCardsExpanded.value); // 注意：当isAllCardsExpanded为true时，卡片应展开，即isCollapsed为false
    }
  };

  // 设置特定卡片的状态
  const setCardState = (cardId, isCollapsed) => {
    cardStates.set(cardId, isCollapsed);
  };

  // 获取特定卡片的状态
  const getCardState = (cardId) => {
    return cardStates.get(cardId) ?? true; // 默认为收起状态
  };

  // 重置为默认状态（全部收起）
  const resetAllCards = () => {
    isAllCardsExpanded.value = false;
    for (let [key, value] of cardStates) {
      cardStates.set(key, true); // 设置为收起状态
    }
  };

  return {
    isAllCardsExpanded,
    cardStates,
    toggleAllCards,
    setCardState,
    getCardState,
    resetAllCards
  };
}

// 提供全局状态
export function provideGlobalCardState() {
  const globalState = createGlobalCardState();
  provide(globalCardStateSymbol, globalState);
  return globalState;
}

// 注入全局状态
export function useGlobalCardState() {
  return inject(globalCardStateSymbol);
}