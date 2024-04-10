import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {RootStackParamList} from '@constants/routes';
import LoginScreen from '@screens/auth/screens/Login.scene';
import UnverifiedScreen from '@screens/verification/screens/Unverified.scene';
import FillPersonalInformationScreen from '@screens/verification/screens/FillPersonalInformation.scene';
import VerifiedScreen from '@screens/verification/screens/Verified.scene';
import MainNavigation from './MainNavigation';
import ConsentCodeScreen from '@screens/home/screens/ConsentCode.scene';
import OnboardingScreen from '@screens/on_boarding/screens/Onboarding.scene';

const Stack = createNativeStackNavigator<RootStackParamList>();

const AppNavigation = () => {
  return (
    <Stack.Navigator initialRouteName={'Onboarding'}>
      <Stack.Screen
        name={'Onboarding'}
        component={OnboardingScreen}
        options={{
          headerShown: false,
        }}
      />
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
      <Stack.Screen
        name={'MainNavigation'}
        component={MainNavigation}
        options={{
          headerShown: false,
        }}
      />
      <Stack.Screen
        name={'ConsentCode'}
        component={ConsentCodeScreen}
        options={{
          headerShown: false,
        }}
      />
    </Stack.Navigator>
  );
};

export default AppNavigation;
