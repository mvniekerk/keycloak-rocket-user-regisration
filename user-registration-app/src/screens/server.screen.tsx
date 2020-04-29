import React, { useState } from 'react';
import { Button, Text, TextInput, View } from 'react-native';
import { useSelector, useDispatch } from 'react-redux';
import { useNavigation } from '@react-navigation/native';
import { setServer as setServerGlobal } from '../reducers/register.reducer';
import { textInputStyle } from './shared';

export const Server: React.FC = () => {
    const dispatch = useDispatch();
    const navigation = useNavigation();
    const [server, setServer] = useState("http://192.168.1.101:8000");

    const onNext = () => {
        dispatch(setServerGlobal(server));
        navigation.navigate('CellNumber');
    };

    const onServerChange = t => setServer(t);
    return (
        <View style={{ padding: 16 }}>
            <Text>Server</Text>
            <TextInput onChangeText={onServerChange} value={server} style={textInputStyle}/>
            <Button title="Next" onPress={onNext}/>
        </View>
    );
}
