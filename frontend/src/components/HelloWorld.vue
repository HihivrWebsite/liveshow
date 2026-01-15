<template>
  <div class="hello-world">
    <h1>{{ msg }}</h1>
    <p>Welcome to Your Vue.js App</p>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld',
  props: {
    msg: String
  }
}
</script>

<style scoped>
.hello-world {
  text-align: center;
  padding: 20px;
  color: #FFC633;
}

.hello-world h1 {
  font-size: 2rem;
  margin-bottom: 10px;
}
</style>