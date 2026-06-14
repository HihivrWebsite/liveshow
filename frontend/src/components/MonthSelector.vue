<template>
  <GlassDialog
    :visible="visible"
    :title="title"
    width="420px"
    @close="$emit('cancel')"
    @update:visible="(v) => !v && $emit('cancel')"
  >
    <div class="modal-form">
      <template v-if="mode === 'single'">
        <div class="form-group">
          <label class="form-label">选择月份:</label>
          <GlassInput
            type="month"
            v-model="singleMonth"
            :placeholder="'请选择月份'"
          />
        </div>
      </template>
      <template v-else>
        <div class="form-group">
          <label class="form-label">起始月份:</label>
          <GlassInput
            type="month"
            v-model="rangeStart"
            :placeholder="'请选择起始月份'"
          />
        </div>
        <div class="form-group">
          <label class="form-label">结束月份:</label>
          <GlassInput
            type="month"
            v-model="rangeEnd"
            :placeholder="'请选择结束月份'"
          />
        </div>
      </template>
    </div>

    <template #footer>
      <GlassButton variant="secondary" @click="handleConfirm">
        <Check :size="16" /> 确定
      </GlassButton>
      <GlassButton variant="default" @click="$emit('cancel')">
        <X :size="16" /> 取消
      </GlassButton>
    </template>
  </GlassDialog>
</template>

<script setup>
import { ref, watch } from 'vue'
import { getCurrentYearMonth, MIN_MONTH } from '@/utils/monthUtils'
import GlassDialog from '@/components/ui/GlassDialog.vue'
import GlassButton from '@/components/ui/GlassButton.vue'
import GlassInput from '@/components/ui/GlassInput.vue'
import { Check, X } from 'lucide-vue-next'

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

.form-label {
  color: var(--color-text-main);
  font-weight: bold;
  font-size: 0.95rem;
}
</style>
