import 'react-native-gesture-handler';
import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { Provider } from 'react-redux';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';

import initStore from './src/redux/store';

import { Server } from './src/screens/server.screen';
import { CellNumber } from './src/screens/cellnumber.screen';
import { OtpVerify } from './src/screens/otp_verify.screen';
import { UserEntry } from './src/screens/user_entry.screen';
import { UserRegistration } from './src/screens/user_registration.screen';
import { UserRegistrationDone } from './src/screens/user_registration_done.screen';
import { OtpEntry } from './src/screens/otp_entry.screen';

export const store = initStore();

const HomeStack = createStackNavigator();

function App() {
  return (
      <NavigationContainer>
        <Provider store={store}>
          <HomeStack.Navigator initialRouteName="Server">
            <HomeStack.Screen name="CellNumber" component={CellNumber} options={{ title: 'Cellphone'}}/>
            <HomeStack.Screen name="OtpEntry" component={OtpEntry} options={{ title: 'OTP'}}/>
            <HomeStack.Screen name="OtpVerify" component={OtpVerify} options={{ header: undefined }}/>
            <HomeStack.Screen name="Server" component={Server} options={{ title: 'Registration Server'}}/>
            <HomeStack.Screen name="UserEntry" component={UserEntry} options={{ title: 'User', headerBackTitleVisible: false}}/>
            <HomeStack.Screen name="UserRegistration" component={UserRegistration} options={{ header: undefined }}/>
            <HomeStack.Screen name="UserRegistrationDone" component={UserRegistrationDone} options={{ title: 'Done' }}/>
          </HomeStack.Navigator>
        </Provider>
      </NavigationContainer>
  );
}

export default App;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});
