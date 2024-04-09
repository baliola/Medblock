import PrimaryButton from '@components/button/PrimaryButton';
import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import AppBarWithIcon from '@components/top_bar/AppBarWithIcon';
import ProfileBar from '@components/top_bar/ProfileBar';
import Images from '@constants/images';
import {RootStackParamList} from '@constants/routes';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {styled} from 'nativewind';
import React, {useState} from 'react';

import {FlatList, ScrollView, TouchableOpacity, View} from 'react-native';

const StyledView = styled(View);
const StyledTouchableOpacity = styled(TouchableOpacity);
const StyledScrollView = styled(ScrollView);
const StyledFlatListMyEmr = styled(FlatList as new () => FlatList<any>);

const data = [{id: 1, title: 'My EMR'}];

const HomeScreen = () => {
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();

  const [isEmrAvailable, _] = useState<boolean>(true);

  const renderItem = ({item}: {item: any}) => (
    <StyledTouchableOpacity
      onPress={() => {
        navigation.navigate('EmrDetail');
      }}
      className="w-1/4 mx-2 flex flex-col items-center bg-slate-200 rounded-xl p-4">
      <GeneralImage url={Images.emr} size={32} />
      <TextPrimary text={item.title} classStyle="text-gray-800 mt-2" isBold />
    </StyledTouchableOpacity>
  );

  const handleShareCode = () => {
    navigation.navigate('ConsentCode');
  };

  return (
    <Scaffold
      topBar={
        <AppBarWithIcon
          child={
            <ProfileBar
              onPressTrailing={() => {
                navigation.navigate('Notification');
              }}
              trailingButton={<GeneralImage url={Images.notif} size={32} />}
            />
          }
        />
      }
      bottomChild={
        <StyledView className="p-6">
          <PrimaryButton
            child={
              <StyledView className="flex flex-row justify-center space-x-2 items-center">
                <GeneralImage url={Images.share} size={24} />
                <TextPrimary
                  text={Strings.label.shareCode}
                  classStyle="text-white"
                  isBold
                />
              </StyledView>
            }
            onPress={() => {
              handleShareCode();
            }}
          />
        </StyledView>
      }>
      <StyledScrollView className="" showsVerticalScrollIndicator={false}>
        {isEmrAvailable ? (
          <StyledView className="w-full items-center px-4">
            <StyledFlatListMyEmr
              className="w-full"
              scrollEnabled={false}
              showsVerticalScrollIndicator={false}
              numColumns={3}
              data={data}
              renderItem={renderItem}
              keyExtractor={item => item.id}
            />
          </StyledView>
        ) : (
          <StyledView className="flex flex-col items-center mt-10 px-6">
            <GeneralImage url={Images.emrEmpty} size={280} />
            <TextPrimary
              text={Strings.label.emptyEmr}
              classStyle="text-gray-800 text-lg"
              isBold
            />
            <TextPrimary
              text={Strings.message.emptyEmr}
              classStyle="text-gray-800"
            />
          </StyledView>
        )}
      </StyledScrollView>
    </Scaffold>
  );
};

export default HomeScreen;
