import PrimaryButton from '@components/button/PrimaryButton';
import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import Images from '@constants/images';
import {RootStackParamList} from '@constants/routes';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {styled} from 'nativewind';
import React, {useState} from 'react';
import {useTranslation} from 'react-i18next';
import {
  FlatList,
  ScrollView,
  StatusBar,
  TouchableOpacity,
  View,
} from 'react-native';

const StyledView = styled(View);
const StyledTouchableOpacity = styled(TouchableOpacity);
const StyledScrollView = styled(ScrollView);
const StyledFlatListMyEmr = styled(FlatList as new () => FlatList<any>);

const data = [{id: 1, title: 'My EMR'}];

const HomeScreen = () => {
  const {t} = useTranslation('global');
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();

  const [isEmrAvailable, _] = useState<boolean>(true);

  const renderItem = ({item}: {item: any}) => (
    <StyledTouchableOpacity
      onPress={() => {}}
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
        <StyledView
          className="flex flex-row justify-between m-6 pt-4 items-center"
          style={{marginTop: StatusBar.currentHeight}}>
          <StyledView className="flex flex-row space-x-4 items-center">
            <GeneralImage size={50} url={Images.dummyProfile} />
            <StyledView className="flex flex-col items-start">
              <TextPrimary
                text="I Putu Aryadi"
                classStyle="text-gray-800 text-lg"
                isBold
              />
              <StyledView className="flex flex-row items-start space-x-2">
                <GeneralImage url={Images.male} size={18} />
                <TextPrimary text="24 th" classStyle="text-gray-800" isBold />
                <TextPrimary text="Maried" classStyle="text-gray-800" />
              </StyledView>
              <TextPrimary
                text={t(Strings.label.emrId, {
                  id: '234564213478',
                })}
              />
            </StyledView>
          </StyledView>

          <GeneralImage url={Images.notif} size={32} />
        </StyledView>
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
