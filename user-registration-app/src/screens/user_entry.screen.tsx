import React, { useState } from 'react';
import { Button, ScrollView, Text, TextInput, View } from 'react-native';
import { textInputStyle } from './shared';
import { useDispatch } from 'react-redux';
import { useNavigation } from '@react-navigation/native';
import { createUser } from '../reducers/register.reducer';

export const UserEntry: React.FC = () => {
    const navigation = useNavigation();
    const dispatch = useDispatch();

    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [passwordConfirm, setPasswordConfirm] = useState("");
    const [firstName, setFirstName] = useState("");
    const [lastName, setLastName] = useState("");
    const [email, setEmail] = useState("");

    const onUsernameChanged = u => setUsername(u);
    const onPasswordChanged = u => setPassword(u);
    const onPasswordConfirmChanged = u => setPasswordConfirm(u);
    const onFirstNameChanged = u => setFirstName(u);
    const onLastNameChanged = u => setLastName(u);
    const onEmailChanged = u => setEmail(u);

    const register = () => {
        dispatch(createUser(username, password, firstName, lastName, email));
        navigation.navigate('UserRegistration');
    };

    return (
        <ScrollView>
            <View style={{ padding: 16 }}>
                <Text>Username</Text>
                <TextInput style={textInputStyle} onChangeText={onUsernameChanged} value={username}/>

                <Text>Password</Text>
                <TextInput style={textInputStyle} onChangeText={onPasswordChanged} value={password} textContentType="password"/>

                <Text>Confirm password</Text>
                <TextInput style={textInputStyle} onChangeText={onPasswordConfirmChanged} value={passwordConfirm} textContentType="password"/>

                <Text>First name</Text>
                <TextInput style={textInputStyle} onChangeText={onFirstNameChanged} value={firstName}/>

                <Text>Last name</Text>
                <TextInput style={textInputStyle} onChangeText={onLastNameChanged} value={lastName}/>

                <Text>Email</Text>
                <TextInput style={textInputStyle} onChangeText={onEmailChanged} value={email}/>

                <Button title={'Register'} onPress={register}/>
            </View>
        </ScrollView>
    );
};
