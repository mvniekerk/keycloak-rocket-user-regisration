import axios from 'axios';
import Storage from './storage-utils';
import Config from './constants';

export const AUTH_TOKEN_KEY = 'jhi-authenticationToken';
export const HOSTNAME = 'hostname';

const TIMEOUT = 1000000; // 10000
const setupAxiosInterceptors = () => {
  const onRequestSuccess = async config => {
    const token = await Storage.local.get(AUTH_TOKEN_KEY);
    config.headers.Authorization = !!token ? `Bearer ${token}` : null;
    config.timeout = TIMEOUT;
    if (!config.ranOverride) {
      config.url = `${Config.SERVER_API_URL}${config.url}`;
      config.ranOverride = true;
    }
    return config;
  };
  const onResponseSuccess = response => response;
  const onResponseError = err => {
    const status = err.status || (err.response && err.response.status);
    console.log("Error: ", status, err, );
    return Promise.reject(err);
  };
  axios.interceptors.request.use(onRequestSuccess);
  axios.interceptors.response.use(onResponseSuccess, onResponseError);
};

export default setupAxiosInterceptors;
