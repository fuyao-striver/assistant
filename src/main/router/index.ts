import { createRouter, createWebHashHistory } from "vue-router";

const pageComponents = import.meta.glob("../views/**/index.vue");

const routes = Object.entries(pageComponents).map(([path, component]) => {
  const match = path.match(/\/(\w+)\/\w+\.vue/);
  const pathText = match?.[1] ?? "";
  return {
    path: "/" + pathText,
    name: pathText,
    component,
  };
});

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/home",
    },
    ...routes,
  ],
});

export default router;
