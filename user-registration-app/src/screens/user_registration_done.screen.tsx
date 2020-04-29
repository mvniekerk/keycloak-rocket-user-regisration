import React from 'react';
import { Text, View } from 'react-native';

import { useSelector } from 'react-redux';
import { IRootState } from '../reducers';

export const UserRegistrationDone: React.FC = () => {
    const userUuid: string | undefined = useSelector((i: IRootState) => i.register.userUuid);
    return (
        <View>
            <Text>Done registration</Text>
            <Text>UUID: {userUuid}</Text>
        </View>
    );
};
