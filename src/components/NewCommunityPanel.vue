<script setup lang="ts">
const emit = defineEmits(['onClose'])

import InputIcon from 'primevue/inputicon';
import IconField from 'primevue/iconfield';
import InputText from 'primevue/inputtext';
import Dropdown from 'primevue/dropdown';
import InputSwitch from 'primevue/inputswitch';
import Button from 'primevue/button';
import { useField, useForm } from 'vee-validate';
import { ref } from 'vue';
import { useSessionsStore } from '../stores/sessions';
import { useToast } from 'primevue/usetoast';

const sessionsStore = useSessionsStore()
const toast = useToast()

const { handleSubmit: handleCheck, resetForm: _resetCheckForm } = useForm();
const { value: name, errorMessage: invalidName } = useField('name', validateName);

function validateName(value: any) {
    if (!value) {
        return "社群名称不应为空！"
    }

    if (value.length > 8) {
        return "社群名称应当小于8个字符！"
    }
    return true;
}

const selectedSecurityLevel = ref();
const crossOrigin = ref(true)
const inProgress = ref(false)
const token = ref("")

const security_levels = ref([
    { name: '开放', code: 0 },
    { name: '安全', code: 1 },
    { name: '私密', code: 2 },
    { name: '绝密', code: 3 },
]);

const onSubmit = handleCheck(async () => {
    inProgress.value = true
    const res = await sessionsStore.newCommunity(name.value, selectedSecurityLevel.value?.code, crossOrigin.value, token.value)
    if (res.status) {
        toast.add({ 'severity': 'success', 'summary': '成功', 'detail': res.msg, 'life': 3000 })
        emit("onClose")
    } else {
        inProgress.value = false;
        toast.add({ 'severity': 'error', 'summary': '失败', 'detail': res.msg, 'life': 3000 })
    }
})
</script>

<template>
    <div class="flex flex-column justify-content-center align-items-center">
        <form @submit="onSubmit" class="flex flex-column gap-3">
            <IconField class="w-full">
                <InputIcon class="pi pi-globe"></InputIcon>
                <InputText id="name-input" v-model="name" class="w-full" placeholder="社群名称" />
            </IconField>
            <small v-if="invalidName" class="p-error" id="name-error">{{ invalidName
                }}</small>
            <Dropdown v-model="selectedSecurityLevel" :options="security_levels" optionLabel="name" placeholder="加密等级"
                checkmark :highlightOnSelect="false" class="w-full" />
            <IconField class="w-full" v-if="selectedSecurityLevel && selectedSecurityLevel.code > 0">
                <InputIcon class="pi pi-lock"></InputIcon>
                <InputText id="token-input" v-model="token" class="w-full" placeholder="社群口令" />
            </IconField>
            <div class="inline-flex justify-content-between">
                <div><span>禁止跨域</span></div>
                <InputSwitch v-model="crossOrigin">
                </InputSwitch>
            </div>
            <div class="mt-3 flex justify-content-end">
                <Button type="submit" :icon="(inProgress ? 'pi pi-spin pi-spinner' : 'pi pi-check-square')" label="创建"
                    size="small" iconPos="right" :disabled="inProgress"></Button>
            </div>
        </form>
    </div>
</template>

<style scoped></style>