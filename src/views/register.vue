<script setup lang="ts">
import Button from "primevue/button";
import Checkbox from "primevue/checkbox";
import InputText from "primevue/inputtext";
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Card from 'primevue/card';
import Stepper from 'primevue/stepper';
import StepperPanel from 'primevue/stepperpanel';
import Toast from 'primevue/toast'
import Password from "primevue/password";

import { useField, useForm } from 'vee-validate';
import { ref } from "vue";
import { useRouter } from 'vue-router';
import { useThemeStore } from "../stores/theme";
import { usePrimeVue } from "primevue/config";
import { useToast } from 'primevue/usetoast';
import { useLoginStore } from "../stores/login";

const router = useRouter();
const toast = useToast()
const loginstore = useLoginStore()

const { handleSubmit: handleMail, resetForm: _resetMailForm } = useForm();
const { value: mail, errorMessage: invalidMail } = useField('mail', validateMail);
const { value: accept, errorMessage: invalidAccept } = useField('accept', validateAccept);

const { handleSubmit: handleRegister, resetForm: _resetRegisterForm } = useForm();
const { value: username, errorMessage: invalidUsername } = useField('username', validateUsername);
const { value: nickname, errorMessage: invalidNickname } = useField('nickname', validateNickname);
const { value: password, errorMessage: invalidPassword } = useField('password', validatePassword);
const { value: retype_password, errorMessage: invalidRetypePassword } = useField('retype_password', validateRetypePassword);

const info = ref<string | null>();
const error = ref<string | null>();

function validateMail(value: string) {
  if (!value) {
    return "请输入Tuta邮箱地址！";
  }

  const regex: RegExp = /^[a-zA-Z0-9._%+-]+@(tutanota.com|tuta.com)$/
  if (!value.match(regex)) {
    return "不是合法的Tuta邮箱！"
  }

  return true;
}

function validateUsername(value: string) {
  if (!value) {
    return "请输入用户名！";
  }

  if (value.length > 10) {
    return "用户名不应大于10个字符！"
  }

  return true;
}

function validateNickname(value: string) {
  if (!value) {
    return "请输入昵称！";
  }

  if (value.length > 10) {
    return "昵称不应大于10个字符！"
  }

  return true;
}

function validateAccept(value: boolean) {
  if (!value) {
    return "请阅读并同意浊莲文明协定！";
  }

  return true;
}

function validatePassword(value: string) {
  if (!value) {
    return "请输入密码！";
  }

  return true;
}

function validateRetypePassword(value: string) {
  if (!value) {
    return "请确认密码！";
  }

  if (value !== password.value) {
    return "确认密码与原密码不一致！"
  }

  return true;
}

const checkMail = handleMail(async () => {
  inProgress.value = true
  toast.add({
    'summary': '略过',
    'severity': 'warn',
    'detail': '弦月测试期间不进行邮箱检定，请自行确保你的邮箱输入正确，否则可能会导致账号无法再次登录。',
    'life': 3000
  })
  await new Promise<void>((resolve) => {
    setTimeout(() => {
      resolve()
    }, 3000)
  })
  inProgress.value = false
  active.value = 1
})

const register = handleRegister(async () => {
  inProgress.value = true;
  const res = await loginstore.register(mail.value, username.value, nickname.value, password.value);
  inProgress.value = false;
  if (res.status) {
    info.value = res.msg
    error.value = null
    const callback = await loginstore.login(mail.value, password.value);
    if (callback.status) {
      info.value = "登录成功！"
      error.value = null
      router.push("/dashboard")
    } else {
      info.value = null
      error.value = callback.msg
    }
  } else {
    error.value = res.msg
    info.value = null
  }

})

const themestore = useThemeStore()
const PrimeVue = usePrimeVue()
const active = ref(0)
const inProgress = ref<boolean>(false)
</script>

<template>
  <main class="w-full h-full">
    <Toast style="max-width: 90%;"></Toast>
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
          <StepperPanel header="注册">
            <template #content="{ prevCallback }">
              <Card class="p-3">
                <template #title>
                  <div class="flex justify-content-center">
                    <h2>注册</h2>
                  </div>
                </template>
                <template #content>
                  <div class="flex flex-column justify-content-center gap-2">
                    <form @click="register" class="flex flex-column gap-3">
                      <IconField>
                        <InputIcon class="pi pi-envelope"></InputIcon>
                        <InputText id="mail-input" v-model="mail" class="w-full" placeholder="Tuta 邮箱" disabled />
                      </IconField>
                      <IconField>
                        <InputIcon class="pi pi-user"></InputIcon>
                        <InputText id="username-input" v-model="username" class="w-full" placeholder="用户名" />
                      </IconField>
                      <small v-if="invalidUsername" class="p-error" id="username-error">{{ invalidUsername
                        }}</small>
                      <IconField>
                        <InputIcon class="pi pi-tag"></InputIcon>
                        <InputText id="nickname-input" v-model="nickname" class="w-full" placeholder="昵称" />
                      </IconField>
                      <small v-if="invalidNickname" class="p-error" id="nickname-error">{{ invalidNickname
                        }}</small>
                      <Password v-model="password" type="text" inputId="password-input" class="w-full"
                        aria-describedby="password-error" toggleMask placeholder="密码" />
                      <small v-if="invalidPassword" class="p-error" id="password-error">{{ invalidPassword
                        }}</small>
                      <Password v-model="retype_password" type="text" inputId="retype-password-input" :feedback="false"
                        class="w-full" aria-describedby="retype-password-error" toggleMask placeholder="确认密码" />
                      <small v-if="invalidRetypePassword" class="p-error" id="retype-password-error">{{
            invalidRetypePassword
          }}</small>
                      <small v-if="info" class="p-info" id="text-info">{{ info }}</small>
                      <small v-if="error" class="p-error" id="text-error">{{ error }}</small>
                      <div class="flex pt-4 justify-content-between">
                        <Button label="返回" icon="pi pi-arrow-left" severity="secondary" @click="prevCallback"></Button>
                        <Button label="注册" :icon="(inProgress ? 'pi pi-spin pi-spinner' : 'pi pi-arrow-right')"
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
