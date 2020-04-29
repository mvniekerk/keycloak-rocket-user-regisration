import { combineReducers } from 'redux';
import register, { IRegisterState } from './register.reducer';

export interface IRootState extends IRegisterState {}

const rootReducer = combineReducers<IRootState>({
    register
});

export default rootReducer;
