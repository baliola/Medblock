import {RootStackParamList} from '@constants/routes';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import React from 'react';
import HomeScreen from '../screens/Home.scene';
import EmrDetailScreen from '../screens/EmrDetail.scene';
import ProfileInformationScreen from '../screens/ProfileInformation.scene';
import RevokeAccessScreen from '../screens/RevokeAccess.scene';
import NotificationScreen from '../screens/Notification.scene';

const Stack = createNativeStackNavigator<RootStackParamList>();

const HomeNavigation = () => {
  return (
    <Stack.Navigator initialRouteName={'HomeNavigation'}>
      <Stack.Screen
        name={'HomeNavigation'}
        component={HomeScreen}
        options={{
          headerShown: false,
        }}
      />
      <Stack.Screen
        name={'EmrDetail'}
        component={EmrDetailScreen}
        options={{
          headerShown: false,
        }}
      />
      <Stack.Screen
        name={'ProfileInformation'}
        component={ProfileInformationScreen}
        options={{
          headerShown: false,
        }}
      />
      <Stack.Screen
        name={'RevokeAccess'}
        component={RevokeAccessScreen}
        options={{
          headerShown: false,
        }}
      />
      <Stack.Screen
        name={'Notification'}
        component={NotificationScreen}
        options={{
          headerShown: false,
        }}
      />
    </Stack.Navigator>
  );
};

export default HomeNavigation;
