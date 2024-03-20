<script setup lang="ts">
import Button from "primevue/button";
import Checkbox from "primevue/checkbox";
import InputText from "primevue/inputtext";
import Password from "primevue/password";
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Card from 'primevue/card';
import Stepper from 'primevue/stepper';
import StepperPanel from 'primevue/stepperpanel';

import { invoke } from "@tauri-apps/api/core";
import { useField, useForm } from 'vee-validate';
import { ref } from "vue";
import { useRouter } from 'vue-router';
import { useThemeStore } from "../stores/theme";
import { usePrimeVue } from "primevue/config";
import Steps from "primevue/steps";
import ProgressSpinner from "primevue/progressspinner";

const router = useRouter();

const { handleSubmit, resetForm: _resetForm } = useForm();
const { value: server, errorMessage: invalidServer } = useField('server', validateServer);
const { value: identity, errorMessage: invalidUsername } = useField('username', validateUsername);
const { value: password, errorMessage: invalidPassword } = useField('password', validatePassword);
const { value: accept, errorMessage: invalidAccept } = useField('accept', validateAccept);

const info = ref<string>();
const error = ref<string>();

function validateServer(value: string) {
  if (!value) {
    return "请输入服务端地址！";
  }

  return true;
}

function validateUsername(value: string) {
  if (!value) {
    return "请输入用户名！";
  }

  return true;
}

function validatePassword(value: string) {
  console.log()
  if (!value) {
    return "请输入密钥！";
  }

  return true;
}

function validateAccept(value: boolean) {
  if (!value) {
    return "请阅读并同意浊莲文明协定！";
  }

  return true;
}

const login = handleSubmit(async () => {
  info.value = "登录中...";

  if (!accept.value) {
    invalidAccept.value = "请先阅读并同意浊莲文明协定！";
    return
  }

  let callback: { status: boolean; session_key: string; error: string } = JSON.parse(
    await invoke("login_handler", { server: server.value, identity: identity.value, password: password.value })
  );
  if (callback["status"]) {
    localStorage.setItem("server", server.value);
    localStorage.setItem("isLoggedIn", "true");
    localStorage.setItem("session_key", callback["session_key"]);
    info.value = "登录成功";
    router.push('/');
  } else {
    info.value = "";
    error.value = callback["error"];
  }
})

const themestore = useThemeStore()
const PrimeVue = usePrimeVue()
const active = ref(0)
const checkStatus = ref(0)
const allChecks = ref([
  {
    label: '网络检查'
  },
  {
    label: '安全检查'
  },
])
</script>

<template>
  <main class="w-full h-full">
    <div class="flex flex-column justify-content-center align-items-center h-full w-full">
      <div style="max-width: 100%;">
        <div class="flex w-full justify-content-start align-items-start pl-2">
          <Button @click="themestore.changeTheme(PrimeVue)" v-if="!themestore.dark" icon="pi pi-sun" plain
            text></Button>
          <Button @click="themestore.changeTheme(PrimeVue)" v-else icon="pi pi-moon" plain text></Button>
        </div>

        <Stepper linear v-model:activeStep="active" class="h-full max-w-full" style="width: 618px">
          <StepperPanel header="检查">
            <template #content>
              <Card class="p-3">
                <template #content>
                  <div class="flex flex-column gap-4">
                    <div class="flex justify-content-center">
                      <Steps class="w-full" :model="allChecks" v-model:activeStep="checkStatus" />
                    </div>
                    <div class="flex justify-content-center align-items-center">
                      <ProgressSpinner></ProgressSpinner>
                    </div>
                  </div>
                </template>
              </Card>
            </template>
          </StepperPanel>
          <StepperPanel header="节点">
            <template #content="{ nextCallback }">
              <Card class="p-3">
                <template #title>
                  <div class="flex justify-content-center">
                    <h1>节点</h1>
                  </div>
                </template>
                <template #content>
                  <div class="flex flex-column justify-content-center gap-2">
                    <form @submit="login" class="flex flex-column gap-3">
                      <IconField>
                        <InputIcon class="pi pi-globe"></InputIcon>
                        <InputText id="server-input" v-model="server" class="w-full" placeholder="节点地址" />
                      </IconField>
                      <small v-if="invalidServer" class="p-error" id="username-error">{{ invalidServer
                        }}</small>
                      <div class="field-checkbox">
                        <Checkbox v-model="accept" :binary="true" name="accept" inputId="accept-input"
                          :class="{ 'p-invalid': invalidAccept }" aria-describedby="accept-error" />
                        <label for="accept-input">我已阅读并同意月长石使用协议</label>
                      </div>
                      <small v-if="invalidAccept" class="p-error" id="accept-error">{{ invalidAccept
                        }}</small>
                      <small v-if="info" class="p-info" id="text-info">{{ info }}</small>
                      <small v-if="error" class="p-error" id="text-error">{{ error }}</small>
                      <div class="flex pt-4 justify-content-end">
                        <Button label="登入" icon="pi pi-arrow-right" iconPos="right" @click="nextCallback"></Button>
                      </div>
                    </form>
                  </div>
                </template>
              </Card>
            </template>
          </StepperPanel>
          <StepperPanel header="登入">
            <template #content="{ prevCallback }">
              <Card class="p-3">
                <template #title>
                  <div class="flex justify-content-center">
                    <h1>登入</h1>
                  </div>
                </template>
                <template #content>
                  <div class="flex flex-column justify-content-center gap-2">
                    <form @submit="login" class="flex flex-column gap-3">
                      <IconField>
                        <InputIcon class="pi pi-globe"></InputIcon>
                        <InputText id="server-input" v-model="server" class="w-full" placeholder="节点地址" />
                      </IconField>
                      <small v-if="invalidServer" class="p-error" id="username-error">{{ invalidServer
                        }}</small>
                      <IconField>
                        <InputIcon class="pi pi-user"></InputIcon>
                        <InputText id="username-input" v-model="identity" class="w-full" placeholder="用户名或邮箱" />
                      </IconField>
                      <small v-if="invalidUsername" class="p-error" id="username-error">{{ invalidUsername
                        }}</small>
                      <Password v-model="password" type="text" inputId="password-input" :feedback="false" class="w-full"
                        aria-describedby="password-error" toggleMask placeholder="密码" />
                      <small v-if="invalidPassword" class="p-error" id="password-error">{{ invalidPassword
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
                        <Button label="返回" icon="pi pi-arrow-left" severity="secondary" @click="prevCallback"></Button>
                        <Button label="认证" icon="pi pi-arrow-right" type="submit" iconPos="right"></Button>
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
</style>
