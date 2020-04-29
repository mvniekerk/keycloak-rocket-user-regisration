import React, { useEffect } from 'react';
import { Text, View } from 'react-native';

import { useSelector } from 'react-redux';
import { useNavigation } from '@react-navigation/core';
import { IRootState } from '../reducers';

export const UserRegistration: React.FC = () => {
    const navigation = useNavigation();
    const creating: boolean = useSelector((i: IRootState) => i.register.creatingUser);
    const error: string | undefined = useSelector((i: IRootState) => i.register.creatingUserError);
    const userUuid: string | undefined = useSelector((i: IRootState) => i.register.userUuid);

    useEffect(() => {
        if (error) {
            navigation.navigate('UserEntry');
        } else if (!creating && !!userUuid) {
            navigation.navigate('UserRegistrationDone');
        }
    }, [creating, error, userUuid]);
    return (
        <View>
            <Text>Registering</Text>
        </View>
    );
}
