import { FAILURE, REQUEST, SUCCESS } from './action-type.util';
import axios from 'axios';

let client = axios.create();

export const ACTIONS = {
    SET_SERVER: 'register/SET_SERVER',
    SEND_OTP: 'register/SEND_OTP',
    VERIFY_OTP: 'register/VERIFY_OTP',
    CREATE_USER: 'register/CREATE_USER'
}

interface IState {
    uuid?: string,
    userUuid?: string,
    otp?: string,
    cellNumber?: string
    username?: string,
    firstName?: string,
    lastName?: string
    email?: string,
    password?: string
    otpVerified: boolean,
    otpVerificationError: boolean,
    verifyingOtp: boolean,
    sendingOtpSms: boolean,
    sendingOtpSmsError: boolean,
    creatingUser: boolean,
    creatingUserError?: string,
    registrationServer: string
}

const initialState: IState = {
    otpVerificationError: false,
    sendingOtpSmsError: false,
    otpVerified: false,
    sendingOtpSms: false,
    verifyingOtp: false,
    creatingUser: false,
    registrationServer: 'http://localhost:8083'
};

export type RegisterState = Readonly<IState>;
export interface IRegisterState {
    readonly register: RegisterState
}

export default (state: RegisterState = initialState, action: any): RegisterState => {
    switch (action.type) {
        case REQUEST(ACTIONS.SEND_OTP):
            return {
                ...state,
                otp: undefined,
                otpVerified: false,
                otpVerificationError: false,
                sendingOtpSms: true,
                sendingOtpSmsError: false,
                uuid: undefined,
                cellNumber: undefined
            };
        case FAILURE(ACTIONS.SEND_OTP):
            return {
                ...state,
                sendingOtpSmsError: true,
                sendingOtpSms: false,
                uuid: undefined,
                cellNumber: undefined
            }
        case SUCCESS(ACTIONS.SEND_OTP):
            console.log('Sent otp? ', action.payload)
            return {
                ...state,
                sendingOtpSms: false,
                sendingOtpSmsError: false,
                uuid: action.payload.uuid,
                cellNumber: action.payload.number
            }

        case REQUEST(ACTIONS.VERIFY_OTP):
            return {
                ...state,
                verifyingOtp: true,
                otpVerificationError: false,
                otpVerified: false,
                otp: undefined
            };
        case FAILURE(ACTIONS.VERIFY_OTP):
            return {
                ...state,
                verifyingOtp: false,
                otpVerificationError: true,
                otpVerified: false,
                otp: undefined
            };
        case SUCCESS(ACTIONS.VERIFY_OTP):
            return {
                ...state,
                verifyingOtp: false,
                otpVerificationError: false,
                otpVerified: true,
                otp: action.payload.otp
            };

        case REQUEST(ACTIONS.CREATE_USER):
            return {
                ...state,
                userUuid: undefined,
                creatingUser: true,
                creatingUserError: undefined
            }
        case FAILURE(ACTIONS.CREATE_USER):
            return {
                ...state,
                userUuid: undefined,
                creatingUser: false,
                creatingUserError: action.payload
            };
        case SUCCESS(ACTIONS.CREATE_USER):
            return {
                ...state,
                userUuid: action.payload,
                creatingUser: false,
                creatingUserError: undefined
            };
        case ACTIONS.SET_SERVER:
            client = axios.create({ baseURL: action.payload})
            return {
                ...state,
                registrationServer: action.payload
            }
        default:
            return state;
    }
}

export const setServer = (url: string) => async dispatch =>
    dispatch({
        type: ACTIONS.SET_SERVER,
        payload: url
    });

export const sendOtp = (number: string) => async (dispatch, getState: () => IRegisterState ) => {
    const url = `/phone/register`;
    const body = { number };
    console.log('Going to send otp', url, body, getState().register.registrationServer);
    return dispatch({
        type: ACTIONS.SEND_OTP,
        payload: client.post(url, body, { headers: { 'Content-Type': 'application/json'}})
            .then(r =>  ({ number, uuid: r.data}))
    })
};

export const verifyOtp = (otp: string) => async (dispatch, getState: () => IRegisterState) => {
    let state = getState().register;
    let url = `/phone/register/${state.uuid}`;
    let body = { number: state.cellNumber, otp }
    return dispatch({
        type: ACTIONS.VERIFY_OTP,
        payload: client.put(url, body)
            .then(r => ({otp}))
    });
}

export const createUser = (
    username: string, password: string, first_name: string, last_name: string, email: string
) => async (dispatch, getState: () => IRegisterState) => {
    let state = getState().register;
    let url = `/user/register/${state.uuid}`
    let body = { username, password, first_name, last_name, email };
    return dispatch({
        type: ACTIONS.CREATE_USER,
        payload: client.post(url, body)
            .then(r => r.data)
    });
}
