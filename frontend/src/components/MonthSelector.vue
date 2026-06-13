<template>
  <div v-if="visible" class="modal-overlay" @click="$emit('cancel')">
    <div class="modal-content" @click.stop>
      <h3>{{ title }}</h3>
      <div class="modal-form">
        <template v-if="mode === 'single'">
          <div class="form-group">
            <label>选择月份:</label>
            <input type="month" v-model="singleMonth" class="month-input" :min="min">
          </div>
        </template>
        <template v-else>
          <div class="form-group">
            <label>起始月份:</label>
            <input type="month" v-model="rangeStart" class="month-input" :min="min">
          </div>
          <div class="form-group">
            <label>结束月份:</label>
            <input type="month" v-model="rangeEnd" class="month-input" :min="min">
          </div>
        </template>
        <div class="button-group">
          <button @click="handleConfirm" class="confirm-btn">确定</button>
          <button @click="$emit('cancel')" class="cancel-btn">取消</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import { getCurrentYearMonth, MIN_MONTH } from '@/utils/monthUtils'

const props = defineProps({
  visible: Boolean,
  title: { type: String, default: '选择月份' },
  mode: { type: String, default: 'single' },
  min: { type: String, default: MIN_MONTH }
})

const emit = defineEmits(['confirm', 'cancel'])

const singleMonth = ref('')
const rangeStart = ref('')
const rangeEnd = ref('')

watch(() => props.visible, (val) => {
  if (val) {
    const current = getCurrentYearMonth()
    singleMonth.value = current
    rangeStart.value = current
    rangeEnd.value = current
  }
})

function handleConfirm() {
  if (props.mode === 'single') {
    if (!singleMonth.value) {
      alert('请选择月份')
      return
    }
    emit('confirm', { selectedMonth: singleMonth.value.replace('-', '') })
  } else {
    if (!rangeStart.value || !rangeEnd.value) {
      alert('请选择起始和结束月份')
      return
    }
    if (rangeStart.value > rangeEnd.value) {
      alert('起始月份不能晚于结束月份')
      return
    }
    emit('confirm', {
      startMonth: rangeStart.value.replace('-', ''),
      endMonth: rangeEnd.value.replace('-', '')
    })
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: #FFF8E1;
  border: 2px solid #FFC633;
  border-radius: 20px;
  padding: 30px;
  min-width: 350px;
  max-width: 450px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
}

.modal-content h3 {
  color: #FFC633;
  text-align: center;
  margin-bottom: 20px;
  font-size: 1.3rem;
}

.modal-form {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.form-group label {
  color: #333;
  font-weight: bold;
  font-size: 0.95rem;
}

.month-input {
  width: 100%;
  padding: 10px;
  border: 2px solid #FFC633;
  border-radius: 10px;
  font-size: 1rem;
  box-sizing: border-box;
}

.button-group {
  display: flex;
  gap: 10px;
  justify-content: center;
  margin-top: 10px;
}

.confirm-btn,
.cancel-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 15px;
  cursor: pointer;
  font-weight: bold;
  transition: all 0.3s ease;
}

.confirm-btn {
  background: linear-gradient(45deg, #f9729a, #f75982);
  color: white;
}

.cancel-btn {
  background: linear-gradient(45deg, #6c757d, #5a6268);
  color: white;
}

.confirm-btn:hover,
.cancel-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}
</style>
