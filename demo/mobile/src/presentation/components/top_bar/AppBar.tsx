import Colors from '@constants/colors';
import Images from '@constants/images';
import {RootStackParamList} from '@constants/routes';
import {StackActions, useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {styled} from 'nativewind';
import React from 'react';
import {Image, StatusBar, TouchableOpacity, View} from 'react-native';

const StyledView = styled(View);
const StyledImage = styled(Image);
const StyledTouchableOpacity = styled(TouchableOpacity);

interface AppBarProps {
  title?: React.ReactElement<any, any>;
  inverse?: boolean;
  trailing?: React.ReactElement<any, any>;
}

const AppBar: React.FC<AppBarProps> = ({title, inverse, trailing}) => {
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();

  const handleGoBack = () => {
    if (navigation.canGoBack()) {
      navigation.goBack();
    } else {
      const replaceAction = StackActions.replace('MainNavigation');
      navigation.dispatch(replaceAction);
    }
  };

  return (
    <StyledView
      style={{marginTop: StatusBar.currentHeight}}
      className="px-4 pt-6 flex flex-row justify-between items-center w-full">
      <StyledTouchableOpacity onPress={handleGoBack} className="h-7 w-7">
        <StyledImage
          tintColor={inverse ? Colors.white : Colors.gray_dark}
          source={parseInt(Images.arrowLeft, 10)}
          className="h-full w-full"
        />
      </StyledTouchableOpacity>

      <StyledView className="flex-1 pl-4">{title}</StyledView>
      <StyledView className="px-2">{trailing}</StyledView>
    </StyledView>
  );
};

export default AppBar;
