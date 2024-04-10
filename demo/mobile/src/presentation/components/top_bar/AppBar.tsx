import Colors from '@constants/colors';
import Images from '@constants/images';
import {RootStackParamList} from '@constants/routes';
import {StackActions, useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {styled} from 'nativewind';
import React from 'react';
import {
  Image,
  StatusBar,
  StyleProp,
  TouchableOpacity,
  View,
  ViewStyle,
} from 'react-native';

const StyledView = styled(View);
const StyledImage = styled(Image);
const StyledTouchableOpacity = styled(TouchableOpacity);

interface AppBarProps {
  title?: React.ReactElement<any, any>;
  inverse?: boolean;
  trailing?: React.ReactElement<any, any>;
  classStyle?: string;
  style?: StyleProp<ViewStyle>;
}

const AppBar: React.FC<AppBarProps> = ({
  title,
  inverse,
  trailing,
  classStyle,
  style = {marginTop: StatusBar.currentHeight},
}) => {
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
      style={style}
      className={
        'px-4 pt-6 flex flex-row justify-between items-center w-full ' +
        classStyle
      }>
      {title ? (
        <StyledTouchableOpacity onPress={handleGoBack} className="h-7 w-7">
          <StyledImage
            tintColor={inverse ? Colors.white : Colors.gray_dark}
            source={parseInt(Images.arrowLeft, 10)}
            className="h-full w-full"
          />
        </StyledTouchableOpacity>
      ) : null}

      <StyledView className="flex-1 pl-4">{title}</StyledView>
      <StyledView className="px-2">{trailing}</StyledView>
    </StyledView>
  );
};

export default AppBar;
