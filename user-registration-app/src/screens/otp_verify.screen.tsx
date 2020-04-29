import React, { useEffect } from 'react';
import { Text, View } from 'react-native';
import { useSelector } from 'react-redux';
import { useNavigation } from '@react-navigation/core';
import { IRootState } from '../reducers';

export const OtpVerify: React.FC = () => {
    const navigation = useNavigation();
    const verifying: boolean = useSelector((i: IRootState) => i.register.verifyingOtp);
    const verified: boolean = useSelector((i: IRootState) => i.register.otpVerified);
    const error: boolean = useSelector((i: IRootState) => i.register.otpVerificationError);

    useEffect(() => {
        if (error) {
            navigation.navigate('OtpEntry');
        } else if (!verifying && verified) {
            navigation.navigate('UserEntry');
        }
    }, [verified, verifying, error]);
    return (
        <View>
            <Text>Verifying OTP</Text>
        </View>
    );
}
