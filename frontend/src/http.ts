import axios from 'axios'
import { apiURL } from '@/config'

const axiosInstance = axios.create({
  baseURL: apiURL,
  headers: {
    'Content-Type': 'application/json',
    Accept: 'application/json'
  },
  withCredentials: true
})
axiosInstance.interceptors.request.use((config) => {
  return config
})
export default axiosInstance
