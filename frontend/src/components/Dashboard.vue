<template>
  <div class="container mt-5">
    <h1>Dashboard</h1>
    <div v-if="user">
      <p>Welcome, {{ user.sub }}!</p>
      <p>Your session expires at: {{ new Date(user.exp * 1000).toLocaleString() }}</p>
      <button @click="logout" class="btn btn-danger">Logout</button>
    </div>
    <div v-else>
      <p>Loading...</p>
    </div>
  </div>
</template>

<script>
import api from '../api'

export default {
  data() {
    return {
      user: null
    }
  },
  async created() {
    try {
      // Interceptor handles attaching token
      const response = await api.get('/api/me')
      this.user = response.data
    } catch (err) {
      // Interceptor handles redirection on failure if refresh fails
      // But we might want to ensure we redirect if it wasn't a 401
      // or if component logic expects it.
      // The interceptor pushes to /login on 401 failure.
      // So this catch might just log or handle other errors.
    }
  },
  methods: {
    logout() {
      localStorage.removeItem('access_token')
      localStorage.removeItem('refresh_token')
      this.$router.push('/login')
    }
  }
}
</script>
