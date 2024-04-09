import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {RootStackParamList} from '@constants/routes';
import HomeScreen from '@screens/home/Home.scene';
import LoginScreen from '@screens/auth/screens/Login.scene';
import UnverifiedScreen from '@screens/verification/screens/Unverified.scene';
import FillPersonalInformationScreen from '@screens/verification/screens/FillPersonalInformation.scene';
import VerifiedScreen from '@screens/verification/screens/Verified.scene';

const Stack = createNativeStackNavigator<RootStackParamList>();

const AppNavigation = () => {
  return (
    <Stack.Navigator initialRouteName={'Login'}>
      <Stack.Screen
        name={'Login'}
        component={LoginScreen}
        options={{
          headerShown: false,
        }}
      />
      <Stack.Screen
        name={'Unverified'}
        component={UnverifiedScreen}
        options={{
          headerShown: false,
        }}
      />
      <Stack.Screen
        name={'Verified'}
        component={VerifiedScreen}
        options={{
          headerShown: false,
        }}
      />
      <Stack.Screen
        name={'FillPersonalInformation'}
        component={FillPersonalInformationScreen}
        options={{
          headerShown: false,
        }}
      />
      <Stack.Screen name={'Home'} component={HomeScreen} />
    </Stack.Navigator>
  );
};

export default AppNavigation;
