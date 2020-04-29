import { createStore, applyMiddleware, compose } from 'redux';
import promiseMiddleware from 'redux-promise-middleware';
import thunkMiddleware from 'redux-thunk';
import reducer, { IRootState } from '../reducers';
import setupAxiosInterceptors from './axios-interceptor';

const defaultMiddlewares = [
  thunkMiddleware,
  // notificationMiddleware,
  promiseMiddleware,
  // loadingBarMiddleware(),
  // websocketMiddleware,
];
const composedMiddlewares = middlewares =>
  compose(applyMiddleware(...defaultMiddlewares, ...middlewares));

const initialize = (initialState?: IRootState, middlewares = []) => createStore(reducer, initialState, composedMiddlewares(middlewares));
setupAxiosInterceptors();
export default initialize;
