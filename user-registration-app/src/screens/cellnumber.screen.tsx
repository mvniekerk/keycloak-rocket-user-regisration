import React, { useState } from 'react';
import { Button, Text, TextInput, View } from 'react-native';
import { textInputStyle } from './shared';
import { useDispatch } from 'react-redux';
import { sendOtp } from '../reducers/register.reducer';
import { useNavigation } from '@react-navigation/native';

export const CellNumber: React.FC = () => {
    const dispatch = useDispatch();
    const navigation = useNavigation();
    const [cellNumber, setCellNumber] = useState("");

    const verify = () => {
        dispatch(sendOtp(cellNumber));
        navigation.navigate('OtpEntry');
    };

    const onCellNumberChange = x => setCellNumber(x);
    return (
        <View style={{ padding: 16 }}>
            <Text>Cellphone number</Text>
            <TextInput style={textInputStyle} value={cellNumber} onChangeText={onCellNumberChange}/>
            <Button title={"Send OTP"} onPress={verify}/>
        </View>
    );
};
