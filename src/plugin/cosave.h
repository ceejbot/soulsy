
namespace cosave
{
	void initializeCosaves();
	void gameSavedHandler(SKSE::SerializationInterface* cosave);
	void revertHandler(SKSE::SerializationInterface* cosave);
	void gameLoadedHandler(SKSE::SerializationInterface* cosave);
}
