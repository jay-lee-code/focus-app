import axios from 'axios'
import router from './router'

const api = axios.create()

api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('access_token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => Promise.reject(error)
)

api.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config

    // If 401 and we haven't retried yet
    if (error.response && error.response.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true

      try {
        const refreshToken = localStorage.getItem('refresh_token')
        if (!refreshToken) {
            throw new Error('No refresh token')
        }

        const response = await axios.post('/api/refresh', {
          refresh_token: refreshToken
        })

        const newAccessToken = response.data.access_token
        localStorage.setItem('access_token', newAccessToken)

        // Update the header for the retry
        originalRequest.headers.Authorization = `Bearer ${newAccessToken}`

        // Retry the original request
        return api(originalRequest)
      } catch (refreshError) {
        // Refresh failed - logout
        localStorage.removeItem('access_token')
        localStorage.removeItem('refresh_token')
        router.push('/login')
        return Promise.reject(refreshError)
      }
    }

    return Promise.reject(error)
  }
)

export default api
