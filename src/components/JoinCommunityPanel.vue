<script setup lang="ts">
const emit = defineEmits(['onClose'])

import InputIcon from 'primevue/inputicon';
import IconField from 'primevue/iconfield';
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import { useField, useForm } from 'vee-validate';
import { ref } from 'vue';
import { useSessionsStore } from '../stores/sessions';
import { useToast } from 'primevue/usetoast';

const sessionsStore = useSessionsStore()
const toast = useToast()

const { handleSubmit: handleCheck, resetForm: _resetCheckForm } = useForm();
const { value: community_id, errorMessage: invalidCommunityId } = useField('community_id', validateId);

function validateId(value: any) {
    if (!value) {
        return "社群 ID 不应为空！"
    }

    if (value.length != 20) {
        return "社群 ID 长度不符！"
    }
    return true;
}

const inProgress = ref(false)

const onSubmit = handleCheck(async () => {
    inProgress.value = true
    const res = await sessionsStore.addCommunity(community_id.value)
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
                <InputText id="name-input" v-model="community_id" class="w-full" placeholder="社群 ID" />
            </IconField>
            <small v-if="invalidCommunityId" class="p-error" id="name-error">{{ invalidCommunityId
                }}</small>
            <div class="mt-3 flex justify-content-end">
                <Button type="submit" :icon="(inProgress ? 'pi pi-spin pi-spinner' : 'pi pi-check-square')" label="加入"
                    size="small" iconPos="right" :disabled="inProgress"></Button>
            </div>
        </form>
    </div>
</template>

<style scoped></style>