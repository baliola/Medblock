import React from 'react';
import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';
import {RouteProp} from '@react-navigation/native';
import {styled} from 'nativewind';
import {Image, View} from 'react-native';
import Images from '@constants/images';
import Colors from '@constants/colors';
import Strings from '@constants/strings';
import {useTranslation} from 'react-i18next';
import AppRoutes from '@constants/routes';
import HomeScreen from '@screens/home/screens/Home.scene';
import FileScreen from '@screens/file/screens/File.scene';
import HistoryScreen from '@screens/history/screens/History.scene';
import SettingScreen from '@screens/setting/screens/Setting.scene';
import {HEIGHT} from '@constants/dimensions';
import HomeNavigation from '@screens/home/navigation/HomeNavigation';

const Tab = createBottomTabNavigator();
const StyledView = styled(View);
const StyledImage = styled(Image);

type BottomTabParamList = {
  [key in keyof typeof AppRoutes]: undefined;
};

const MainNavigation = () => {
  const {t} = useTranslation('global');

  const getTabBarIcon = (
    route: RouteProp<BottomTabParamList, keyof BottomTabParamList>,
    focused: boolean,
    color: string,
    size: number,
  ) => {
    let iconName = '';

    switch (route.name) {
      case t(Strings.navigation.home):
        iconName = Images.home;
        break;
      case t(Strings.navigation.file):
        iconName = Images.file;
        break;
      case t(Strings.navigation.history):
        iconName = Images.history;
        break;
      case t(Strings.navigation.setting):
        iconName = Images.setting;
        break;
      default:
        break;
    }

    return (
      <StyledView style={{height: size, width: size}}>
        <StyledImage
          source={parseInt(iconName, 10)}
          className="h-full w-full"
          style={{objectFit: 'scale-down', tintColor: color}}
        />
      </StyledView>
    );
  };

  return (
    <Tab.Navigator
      initialRouteName={t(Strings.navigation.home)}
      screenOptions={({route}) => ({
        headerShown: false,
        tabBarIcon: ({focused, color, size}) =>
          getTabBarIcon(route as any, focused, color, size),
        tabBarActiveTintColor: Colors.primary_normal,
        tabBarInactiveTintColor: Colors.gray,
        tabBarLabelStyle: {fontFamily: 'Sora-Regular', fontSize: 10},
        tabBarStyle: {
          height: HEIGHT * 0.09,
          paddingBottom: 20,
          paddingTop: 10,
          borderTopLeftRadius: 20,
          borderTopRightRadius: 20,
          elevation: 0,
        },
      })}>
      <Tab.Screen
        name={t(Strings.navigation.home)}
        component={HomeNavigation}
      />
      <Tab.Screen name={t(Strings.navigation.file)} component={FileScreen} />
      <Tab.Screen
        name={t(Strings.navigation.history)}
        component={HistoryScreen}
      />
      <Tab.Screen
        name={t(Strings.navigation.setting)}
        component={SettingScreen}
      />
    </Tab.Navigator>
  );
};

export default MainNavigation;
