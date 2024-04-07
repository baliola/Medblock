import React, {useEffect} from 'react';
import {RootStackParamList} from '@constants/routes';
import {NavigationContainer} from '@react-navigation/native';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {Provider} from 'react-redux';
import SplashScreen from 'react-native-splash-screen';
import {store} from './src/presentation/redux/store';
import HomeScreen from './src/presentation/modules/home/Home.scene';
import {I18nextProvider} from 'react-i18next';
import i18n from './i18n';
import {QueryClient, QueryClientProvider} from 'react-query';

const queryClient = new QueryClient();

const Stack = createNativeStackNavigator<RootStackParamList>();
function App(): React.JSX.Element {
  useEffect(() => {
    SplashScreen.hide();
  }, []);

  return (
    <I18nextProvider i18n={i18n}>
      <NavigationContainer>
        <QueryClientProvider client={queryClient}>
          <Provider store={store}>
            <Stack.Navigator>
              <Stack.Screen name={'Home'} component={HomeScreen} />
            </Stack.Navigator>
          </Provider>
        </QueryClientProvider>
      </NavigationContainer>
    </I18nextProvider>
  );
}

export default App;
