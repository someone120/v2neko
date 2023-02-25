<template>
  <div class="activite-bar">
    <div class="activite-item-container" v-for="i in items">
      <span
        :class="{
          'material-icons-outlined': !i.choosed,
          'material-icons': i.choosed,
          'activite-item-unchoice': !i.choosed,
          'activite-item-choiced': i.choosed,
        }"
        class="activite-item"
        @click="click(i)"
        >{{ i.icon }}</span
      >
    </div>
  </div>
  <div class="content" :is="router"></div>
</template>

<script setup lang="ts">
import { Ref, computed, ref } from "vue";
import home from "./content/home.vue"

let routes:{[name:string]:any} = {
  "Home":home
}

let router = computed(() => {
  return routes[choosing]
})

let choosing = "Home";
let items: Ref<{
  [name: string]: { icon: string; title: string; choosed: boolean };
}> = ref({
  Home: { icon: "home", title: "Home", choosed: true },
  Proxys: { icon: "adjust", title: "Proxys", choosed: false },
  Settings: { icon: "settings", title: "Settings", choosed: false },
});

function click(i: { icon: string; title: string; choosed: boolean }) {
  if (choosing == i.title) {
    return;
  }
  i.choosed = true;
  items.value[choosing].choosed = false;
  choosing = i.title;
}
</script>

<style scoped>
.activite-bar {
  float:left;
  display: flex;
  flex-direction: column;
  width: 48px;
  height: 100%;
  background-color: #f596aa;
  align-items: flex-start;
  padding: 8px;
  gap: 10px;
}

.activite-item {
  left: 8px;
  user-select: none;
  font-size: 48px;
}

.activite-item-container {
  width: 48px;
  height: 48px;
}

.activite-item-unchoice {
  color: #A85668;
}

.activite-item-choiced {
  color: white;
}
</style>
