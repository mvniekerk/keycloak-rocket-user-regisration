import React, { useState } from 'react';
import { Button, Text, TextInput, View } from 'react-native';
import { useDispatch } from 'react-redux';
import { useNavigation } from '@react-navigation/native';
import { verifyOtp } from '../reducers/register.reducer';
import { textInputStyle } from './shared';

export const OtpEntry: React.FC = () => {
    const dispatch = useDispatch();
    const navigation = useNavigation();

    const [otp, setOtp] = useState("");
    const onOtpChange = x => setOtp(x);
    const onVerifyClick = () => {
        dispatch(verifyOtp(otp));
        navigation.navigate('OtpVerify');
    };

    return (
        <View style={{ padding: 16 }}>
            <Text>OTP sent via SMS</Text>
            <TextInput value={otp} textContentType="oneTimeCode" onChangeText={onOtpChange} style={textInputStyle}/>
            <Button title={"Verify"} onPress={onVerifyClick}/>
        </View>
    );
};
