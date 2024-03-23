<script setup lang="ts">
import Button from "primevue/button";
import Checkbox from "primevue/checkbox";
import InputText from "primevue/inputtext";
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Card from 'primevue/card';
import Stepper from 'primevue/stepper';
import StepperPanel from 'primevue/stepperpanel';

import { useField, useForm } from 'vee-validate';
import { ref } from "vue";
import { useRouter } from 'vue-router';
import { useThemeStore } from "../stores/theme";
import { usePrimeVue } from "primevue/config";

const router = useRouter();

const { handleSubmit: handleMail, resetForm: _resetMailForm } = useForm();
const { value: mail, errorMessage: invalidMail } = useField('mail', validateMail);
const { value: accept, errorMessage: invalidAccept } = useField('accept', validateAccept);

const info = ref<string | null>();
const error = ref<string | null>();

function validateMail(value: string) {
  if (!value) {
    return "请输入Tuta 邮箱地址！";
  }

  return true;
}

function validateAccept(value: boolean) {
  if (!value) {
    return "请阅读并同意浊莲文明协定！";
  }

  return true;
}

const checkMail = handleMail(async () => {
  inProgress.value = true
})

const themestore = useThemeStore()
const PrimeVue = usePrimeVue()
const active = ref(0)
const inProgress = ref<boolean>(false)
</script>

<template>
  <main class="w-full h-full">
    <div class="flex flex-column justify-content-center align-items-center h-full w-full">
      <div class="max-w-full max-h-full">
        <div class="flex w-full justify-content-start align-items-start pl-2">
          <Button @click="themestore.changeTheme(PrimeVue)" v-if="!themestore.dark" icon="pi pi-sun" plain
            text></Button>
          <Button @click="themestore.changeTheme(PrimeVue)" v-else icon="pi pi-moon" plain text></Button>
        </div>

        <Stepper linear v-model:activeStep="active" class="h-full max-w-full pb-4" style="width: 618px">
          <StepperPanel header="邮箱">
            <template #content>
              <Card class="p-3">
                <template #title>
                  <div class="flex justify-content-center">
                    <h2>邮箱</h2>
                  </div>
                </template>
                <template #content>
                  <div class="flex flex-column justify-content-center gap-2">
                    <form @submit="checkMail" class="flex flex-column gap-3">
                      <IconField>
                        <InputIcon class="pi pi-envelope"></InputIcon>
                        <InputText id="mail-input" v-model="mail" class="w-full" placeholder="Tuta 邮箱" />
                      </IconField>
                      <small v-if="invalidMail" class="p-error" id="mail-error">{{ invalidMail
                        }}</small>
                      <div class="field-checkbox">
                        <Checkbox v-model="accept" :binary="true" name="accept" inputId="accept-input"
                          :class="{ 'p-invalid': invalidAccept }" aria-describedby="accept-error" />
                        <label for="accept-input">我已阅读并同意浊莲文明协定</label>
                      </div>
                      <small v-if="invalidAccept" class="p-error" id="accept-error">{{ invalidAccept
                        }}</small>
                      <small v-if="info" class="p-info" id="text-info">{{ info }}</small>
                      <small v-if="error" class="p-error" id="text-error">{{ error }}</small>
                      <div class="flex pt-4 justify-content-between">
                        <Button label="返回" icon="pi pi-arrow-left" severity="secondary" @click="router.back()"></Button>
                        <Button label="检查" :icon="(inProgress ? 'pi pi-spin pi-spinner' : 'pi pi-arrow-right')"
                          type="submit" iconPos="right"></Button>
                      </div>
                    </form>
                  </div>
                </template>
              </Card>
            </template>
          </StepperPanel>
        </Stepper>
      </div>
    </div>
  </main>
</template>

<style scoped>
:deep(.p-password-input) {
  width: 100%;
}

:deep(.p-stepper .p-stepper-nav) {
  overflow-x: hidden;
}

:deep(.p-steps .p-steps-item .p-menuitem-link .p-steps-title) {
  font-size: 0.9rem;
}
</style>
